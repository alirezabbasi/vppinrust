use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde::Serialize;
use serde_json::{Map, Value};
use vpp_interop::{PacketStatsAdapter, PacketStatsInput, RustPacketStatsAdapter};

type JsonCases = BTreeMap<String, Value>;

#[derive(Debug, Serialize)]
struct ConformanceReport {
    compared_cases: usize,
    pass_cases: usize,
    fail_cases: usize,
    missing_cases: Vec<String>,
    mismatches: Vec<CaseMismatch>,
}

#[derive(Debug, Serialize)]
struct CaseMismatch {
    case: String,
    differences: Vec<String>,
}

fn usage() {
    eprintln!(
        "Usage: cargo run -p vpp-conformance -- \\\n         --c-fixtures <dir> --rust-fixtures <dir> --report-out <path>"
    );
}

fn collect_json(dir: &Path) -> Result<JsonCases, String> {
    let mut result = BTreeMap::new();
    let entries = fs::read_dir(dir)
        .map_err(|err| format!("failed to read fixture dir {}: {err}", dir.display()))?;

    for entry in entries {
        let entry = entry.map_err(|err| format!("failed to read dir entry: {err}"))?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| format!("invalid fixture filename: {}", path.display()))?
            .to_string();

        let content = fs::read_to_string(&path)
            .map_err(|err| format!("failed to read fixture {}: {err}", path.display()))?;
        let value: Value = serde_json::from_str(&content)
            .map_err(|err| format!("invalid json fixture {}: {err}", path.display()))?;
        result.insert(stem, value);
    }

    Ok(result)
}

fn diff_json(path: &str, left: &Value, right: &Value, out: &mut Vec<String>) {
    match (left, right) {
        (Value::Object(lm), Value::Object(rm)) => diff_objects(path, lm, rm, out),
        (Value::Array(la), Value::Array(ra)) => {
            if la.len() != ra.len() {
                out.push(format!("{path}: array length differs ({} != {})", la.len(), ra.len()));
                return;
            }
            for (idx, (lv, rv)) in la.iter().zip(ra.iter()).enumerate() {
                let next = format!("{path}[{idx}]");
                diff_json(&next, lv, rv, out);
            }
        }
        _ => {
            if left != right {
                out.push(format!("{path}: {left} != {right}"));
            }
        }
    }
}

fn diff_objects(
    path: &str,
    left: &Map<String, Value>,
    right: &Map<String, Value>,
    out: &mut Vec<String>,
) {
    let keys: BTreeSet<_> = left.keys().chain(right.keys()).collect();
    for key in keys {
        let next = if path.is_empty() { key.clone() } else { format!("{path}.{key}") };

        match (left.get(key), right.get(key)) {
            (Some(lv), Some(rv)) => diff_json(&next, lv, rv, out),
            (Some(_), None) => out.push(format!("{next}: missing on rust side")),
            (None, Some(_)) => out.push(format!("{next}: missing on c side")),
            (None, None) => {}
        }
    }
}

fn build_rust_from_input(value: &Value) -> Result<Value, String> {
    let input: PacketStatsInput = serde_json::from_value(value.clone())
        .map_err(|err| format!("fixture cannot be parsed as PacketStatsInput: {err}"))?;
    let adapter = RustPacketStatsAdapter;
    let output = adapter.evaluate(&input);
    serde_json::to_value(output).map_err(|err| format!("failed to serialize rust output: {err}"))
}

fn parse_args(args: &[String]) -> Result<(PathBuf, PathBuf, PathBuf), ExitCode> {
    let mut c_dir: Option<PathBuf> = None;
    let mut rust_dir: Option<PathBuf> = None;
    let mut report_out: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--c-fixtures" => {
                i += 1;
                c_dir = args.get(i).map(PathBuf::from);
            }
            "--rust-fixtures" => {
                i += 1;
                rust_dir = args.get(i).map(PathBuf::from);
            }
            "--report-out" => {
                i += 1;
                report_out = args.get(i).map(PathBuf::from);
            }
            _ => {}
        }
        i += 1;
    }

    let (Some(c_dir), Some(rust_dir), Some(report_out)) = (c_dir, rust_dir, report_out) else {
        usage();
        return Err(ExitCode::from(2));
    };
    Ok((c_dir, rust_dir, report_out))
}

fn load_cases(c_dir: &Path, rust_dir: &Path) -> Result<(JsonCases, JsonCases), ExitCode> {
    let c_cases = collect_json(c_dir).map_err(|err| {
        eprintln!("{err}");
        ExitCode::from(1)
    })?;
    let rust_expected_cases = collect_json(rust_dir).map_err(|err| {
        eprintln!("{err}");
        ExitCode::from(1)
    })?;
    Ok((c_cases, rust_expected_cases))
}

fn evaluate_cases(c_cases: &JsonCases, rust_expected_cases: &JsonCases) -> ConformanceReport {
    let case_names: BTreeSet<_> =
        c_cases.keys().chain(rust_expected_cases.keys()).cloned().collect();

    let mut missing_cases = Vec::new();
    let mut mismatches = Vec::new();
    let mut pass_cases = 0usize;

    for case in case_names {
        let Some(c_input) = c_cases.get(&case) else {
            missing_cases.push(format!("{case}: missing C fixture"));
            continue;
        };
        let Some(rust_expected) = rust_expected_cases.get(&case) else {
            missing_cases.push(format!("{case}: missing Rust fixture"));
            continue;
        };

        let rust_actual = match build_rust_from_input(c_input) {
            Ok(value) => value,
            Err(err) => {
                mismatches.push(CaseMismatch { case: case.clone(), differences: vec![err] });
                continue;
            }
        };

        let mut differences = Vec::new();
        diff_json("", rust_expected, &rust_actual, &mut differences);
        if differences.is_empty() {
            pass_cases += 1;
        } else {
            mismatches.push(CaseMismatch { case, differences });
        }
    }

    let fail_cases = mismatches.len() + missing_cases.len();
    let compared_cases = pass_cases + fail_cases;
    ConformanceReport { compared_cases, pass_cases, fail_cases, missing_cases, mismatches }
}

fn write_report(report_out: &Path, report: &ConformanceReport) -> Result<(), ExitCode> {
    if let Some(parent) = report_out.parent()
        && let Err(err) = fs::create_dir_all(parent)
    {
        eprintln!("failed to create report dir {}: {err}", parent.display());
        return Err(ExitCode::from(1));
    }
    let report_json = match serde_json::to_string_pretty(&report) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("failed to serialize report: {err}");
            return Err(ExitCode::from(1));
        }
    };

    if let Err(err) = fs::write(report_out, report_json) {
        eprintln!("failed to write report {}: {err}", report_out.display());
        return Err(ExitCode::from(1));
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let (c_dir, rust_dir, report_out) = match parse_args(&args) {
        Ok(v) => v,
        Err(code) => return code,
    };
    let (c_cases, rust_expected_cases) = match load_cases(&c_dir, &rust_dir) {
        Ok(v) => v,
        Err(code) => return code,
    };
    let report = evaluate_cases(&c_cases, &rust_expected_cases);
    if let Err(code) = write_report(&report_out, &report) {
        return code;
    }

    if report.fail_cases > 0 {
        eprintln!(
            "conformance failed: pass={} fail={} report={}",
            report.pass_cases,
            report.fail_cases,
            report_out.display()
        );
        ExitCode::from(1)
    } else {
        println!(
            "conformance passed: compared={} report={}",
            report.compared_cases,
            report_out.display()
        );
        ExitCode::SUCCESS
    }
}

#[cfg(test)]
mod tests {
    use super::diff_json;
    use serde_json::json;

    #[test]
    fn json_diff_reports_field_path() {
        let left = json!({"a": {"b": 1}});
        let right = json!({"a": {"b": 2}});
        let mut out = Vec::new();
        diff_json("", &left, &right, &mut out);
        assert_eq!(out, vec![String::from("a.b: 1 != 2")]);
    }
}
