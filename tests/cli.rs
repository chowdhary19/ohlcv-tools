use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;

fn csv_fixture(contents: &str) -> tempfile::NamedTempFile {
    let mut file = tempfile::NamedTempFile::new().expect("create temp file");
    write!(file, "{contents}").expect("write fixture contents");
    file
}

#[test]
fn info_prints_description() {
    Command::cargo_bin("ohlcv-tools")
        .unwrap()
        .arg("info")
        .assert()
        .success()
        .stdout(predicate::str::contains("ohlcv-tools"));
}

#[test]
fn no_subcommand_defaults_to_info() {
    Command::cargo_bin("ohlcv-tools")
        .unwrap()
        .assert()
        .success()
        .stdout(predicate::str::contains("VWAP"));
}

#[test]
fn vwap_computes_correct_value() {
    let file = csv_fixture("price,volume\n10,1\n20,3\n");
    Command::cargo_bin("ohlcv-tools")
        .unwrap()
        .arg("vwap")
        .arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("17.5"));
}

#[test]
fn vwap_reads_from_stdin() {
    Command::cargo_bin("ohlcv-tools")
        .unwrap()
        .arg("vwap")
        .arg("-")
        .write_stdin("price,volume\n10,1\n20,3\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("17.5"));
}

#[test]
fn vwap_missing_file_fails_cleanly() {
    Command::cargo_bin("ohlcv-tools")
        .unwrap()
        .arg("vwap")
        .arg("/nonexistent/path/does-not-exist.csv")
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to read"));
}

#[test]
fn sma_prints_series() {
    let file = csv_fixture("price\n1\n2\n3\n4\n");
    Command::cargo_bin("ohlcv-tools")
        .unwrap()
        .arg("sma")
        .arg(file.path())
        .arg("--window")
        .arg("2")
        .assert()
        .success()
        .stdout("1.5\n2.5\n3.5\n");
}

#[test]
fn aggregate_prints_candle_header_and_rows() {
    let file = csv_fixture("timestamp,price,volume\n0,100,1\n30,110,2\n");
    Command::cargo_bin("ohlcv-tools")
        .unwrap()
        .arg("aggregate")
        .arg(file.path())
        .arg("--interval")
        .arg("60")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "timestamp,open,high,low,close,volume\n",
        ));
}

#[test]
fn aggregate_non_positive_interval_fails_cleanly() {
    let file = csv_fixture("timestamp,price,volume\n0,100,1\n");
    Command::cargo_bin("ohlcv-tools")
        .unwrap()
        .arg("aggregate")
        .arg(file.path())
        .arg("--interval")
        .arg("0")
        .assert()
        .failure();
}

#[test]
fn resample_merges_candles_into_larger_interval() {
    let file = csv_fixture(
        "timestamp,open,high,low,close,volume\n\
         0,100,105,98,102,10\n\
         60,102,108,101,107,20\n\
         120,107,107,90,95,15\n",
    );
    Command::cargo_bin("ohlcv-tools")
        .unwrap()
        .arg("resample")
        .arg(file.path())
        .arg("--interval")
        .arg("300")
        .assert()
        .success()
        .stdout("timestamp,open,high,low,close,volume\n0,100,108,90,95,45\n");
}

#[test]
fn resample_reads_from_stdin() {
    Command::cargo_bin("ohlcv-tools")
        .unwrap()
        .arg("resample")
        .arg("-")
        .arg("--interval")
        .arg("300")
        .write_stdin("timestamp,open,high,low,close,volume\n0,100,105,98,102,10\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("0,100,105,98,102,10"));
}

#[test]
fn resample_non_positive_interval_fails_cleanly() {
    let file = csv_fixture("timestamp,open,high,low,close,volume\n0,100,105,98,102,10\n");
    Command::cargo_bin("ohlcv-tools")
        .unwrap()
        .arg("resample")
        .arg(file.path())
        .arg("--interval")
        .arg("0")
        .assert()
        .failure();
}
