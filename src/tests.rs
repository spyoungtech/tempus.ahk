use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, Output};


fn run_script(script_text: String) -> Output {
    let mut proc = Command::new("autohotkeyv2.exe")
        .arg("/CP65001")
        .arg("/ErrorStdout")
        .arg("*")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to spawn autohotkeyv2");
    let mut stdin = proc.stdin.take().expect("failed to open stdin");
    std::thread::scope(|s| {
        s.spawn(|| {
            stdin.write_all(&script_text.as_bytes()).expect("failed to write to stdin");
        });
    });
    proc.wait_with_output().unwrap()
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
    format!("{}\n\n{}", header, script_text)
}

#[test]
fn test_command() {
    run_script("obj := {}".to_string());
}

#[test]
fn test_timestamp_parse() {
    let script = make_script("ts := Timestamp.parse(\"2024-01-01T00:00:00Z\")\nwritestdout(ts.to_string())");
    let output = run_script(script);
    assert!(output.status.success());
}

