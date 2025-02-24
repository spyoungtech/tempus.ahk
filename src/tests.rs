use std::io::{Write};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, Output};


fn run_script(script_text: String) -> Output {
    let mut child = Command::new("autohotkeyv2.exe")
        .arg("/CP65001")
        .arg("/ErrorStdout=UTF-8")
        .arg("*")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(script_text.as_bytes()).expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    output


}

fn get_tempus_ahk_location() -> PathBuf {
    let current_file_path = file!();
    let path = Path::new(current_file_path);
    let grandparent_path = path.parent().and_then(Path::parent).expect("Failed to find grandparent directory");
    grandparent_path.join("tempus.ahk")
}

fn get_dll_location() -> PathBuf {
    let current_file_path = file!();
    let path = Path::new(current_file_path);
    let grandparent_path = path.parent().and_then(Path::parent).expect("Failed to find grandparent directory");
    grandparent_path.join("target").join("x86_64-pc-windows-gnu").join("debug").join("tempus_ahk.dll")
}

fn make_script(script_text: &str) -> String {
    let header = format!("#DllLoad \"{}\"\n#Include \"{}\"\nstdout := FileOpen(\"*\", \"w\", \"UTF-8\")\nwritestdout(message) {{\n    stdout.Write(message)\n    stdout.Read(0)\n}}", get_dll_location().to_str().unwrap(), get_tempus_ahk_location().to_str().unwrap());
    format!("{}\n\n{}\r\n", header, script_text)
}

#[test]
fn test_command() {
    run_script("obj := {}".to_string());
}

#[test]
fn test_timestamp_parse() {
    let script = make_script("ts := Timestamp.parse(\"2024-01-01T00:00:00Z\")\nwritestdout(ts.to_string())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("2024-01-01T00:00:00Z"));
    assert!(output.status.success());
}


#[test]
fn test_timestamp_strptime() {
    let script = make_script("ts := Timestamp.strptime(\"%F %H:%M %:z\", \"2024-07-14 21:14 -04:00\")\nwritestdout(ts.as_second())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1721006040"));
    assert!(output.status.success());
}

#[test]
fn test_timestamp_strftime() {
    let script = make_script("ts := Timestamp.from_second(86400)\nout := ts.strftime(\"%a %b %e %I:%M:%S %p UTC %Y\")\nwritestdout(out)");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("Fri Jan  2 12:00:00 AM UTC 1970"));
    assert!(output.status.success());

}

#[test]
fn test_timestamp_is_zero() {
    let script = make_script("ts := Timestamp.UNIX_EPOCH()\nwritestdout(ts.is_zero())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1"));
    assert!(output.status.success());

    let script = make_script("ts := Timestamp.from_second(1)\nwritestdout(ts.is_zero())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("0"));
    assert!(output.status.success());
}

#[test]
fn test_timestamp_round() {
    let script = make_script("ts := Timestamp.parse(\"2024-06-20 03:25:01Z\")\nrounded := ts.round(Unit.Minute, 1, RoundMode.Ceil)\nwritestdout(rounded.to_string())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("2024-06-20T03:26:00Z"));
    assert!(output.status.success());


}