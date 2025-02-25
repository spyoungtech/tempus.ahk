use std::io::{Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, Output};
use jiff::ToSpan;

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
    let header = format!("\
    #DllLoad \"{}\" \n\
    #Include \"{}\"\n\
    stdout := FileOpen(\"*\", \"w\", \"UTF-8\")\n\
    stderr := FileOpen(\"**\", \"w\", \"UTF-8\")\n\
    writestdout(message) {{\n\
        stdout.Write(message)\n\
        stdout.Read(0)\n\
    }}\
    writestderr(message) {{\n\
        stderr.Write(message)\n\
        stderr.Read(0)\n\
    }}\
    ", get_dll_location().to_str().unwrap(), get_tempus_ahk_location().to_str().unwrap());
    format!("{}\n\
    main(){{\n\
    {}\n\
    }}\n\
    try {{
        main()\n\
    }} catch Any as e {{\n\
        message := Format(\"Error {{}} (line {{}}). The error message was: {{}}. Specifically: {{}}`nStack:`n{{}}\", e.what, e.line, e.message, e.extra, e.stack)\n\
        writestderr(message)\n\
        Exit 1\n\
    }}\n\
    \r\n", header, script_text)
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

#[test]
fn test_span() {
    let script = make_script("sp := Span.new().days(5).hours(8).minutes(1)\nwritestdout(sp.to_string())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("P5DT8H1M"));
    assert!(output.status.success());
}

#[test]
fn test_span_add() {
    let script = make_script("span1 := Span.new().hours(2).minutes(59)\nspan2 := Span.new().minutes(2)\nspan3 := span1.checked_add(span2)\nwritestdout(span3.to_string())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("PT3H1M"));
    assert!(output.status.success());
}

#[test]
fn test_span_sub() {
    let script = make_script("span1 := Span.new().hours(3).minutes(59)\nspan2 := Span.new().minutes(59)\nspan3 := span1.checked_sub(span2)\nwritestdout(span3.to_string())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("PT3H"));
    assert!(output.status.success());
}

#[test]
fn test_span_mul() {
    let script = make_script("span1 := Span.new().hours(3).checked_mul(10)\nwritestdout(span1.to_string())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("PT30H"));
    assert!(output.status.success());
}

#[test]
fn test_span_err() {
    let script = make_script("span1 := Span.new().years(10000).checked_mul(3)");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not in the required range"));
    assert_eq!(stdout.to_string(), String::from(""));
    assert!(!output.status.success());
}

#[test]
fn test_span_compare_eq() {
    let script = make_script("span1 := Span.new().hours(3)\nspan2 := Span.new().minutes(180)\nwritestdout(span1.eq(span2))");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1"));
    assert!(output.status.success());
}


#[test]
fn test_span_compare_gte() {
    let script = make_script("span1 := Span.new().hours(3)\nspan2 := Span.new().minutes(180)\nwritestdout(span1.gte(span2))");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1"));
    assert!(output.status.success());
}

#[test]
fn test_span_compare_lte(){
    let script = make_script("span1 := Span.new().hours(3)\nspan2 := Span.new().minutes(180)\nwritestdout(span1.lte(span2))");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1"));
    assert!(output.status.success());
}

#[test]
fn test_span_compare_lt() {
    let script = make_script("span1 := Span.new().hours(3)\nspan2 := Span.new().minutes(181)\nwritestdout(span1.lt(span2))");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1"));
    assert!(output.status.success());
}

#[test]
fn test_span_compare_gt() {
    let script = make_script("span1 := Span.new().hours(3)\nspan2 := Span.new().minutes(179)\nwritestdout(span1.gt(span2))");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1"));
    assert!(output.status.success());
}

#[test]
fn test_span_compare_fails_non_span() {
    let script = make_script("span1 := Span.new().hours(3)\nspan2 := Timestamp.now()\nwritestdout(span1.gt(span2))");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Only spans can be compared with spans"));
    assert_eq!(stdout.to_string(), String::from(""));
    assert!(!output.status.success());
}

#[test]
fn test_span_compare_fails_calendar_components() {
    let script = make_script("span1 := Span.new().days(300)\nspan2 := Span.new().months(1)\nwritestdout(span1.gt(span2))");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("requires that a relative reference time be given"), "{stderr}");
    assert_eq!(stdout.to_string(), String::from(""));
    assert!(!output.status.success());
}

#[test]
fn test_span_compare_24_hours() {
    let script = make_script("span1 := Span.new().days(30)\nspan2 := Span.new().weeks(3)\nwritestdout(span1.gt(span2, true))");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1"));
    assert!(output.status.success());
}


#[test]
fn test_span_total() {
    let script = make_script("span1 := Span.new().hours(3).minutes(10)\nwritestdout(span1.total(Unit.Second))");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("11400.0"));
    assert!(output.status.success());
}

#[test]
fn test_span_round() {
    let script = make_script("span1 := Span.parse(\"PT23h50m3.123s\")\nexpected := Span.new().hours(24)\nrounded := span1.round(Unit.Minute, 30)\nwritestdout(expected.eq(rounded))");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1"));
    assert!(output.status.success());
}

#[test]
fn test_signed_duration_from_secs() {
    let script = make_script("duration := SignedDuration.from_secs(1.0)\nwritestdout(duration.as_millis())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1000.0"));
    assert!(output.status.success());
}

#[test]
fn test_sign_duration_from_millis() {
    let script = make_script("duration := SignedDuration.from_millis(10)\nwritestdout(duration.as_millis())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("10.0"));
    assert!(output.status.success());

}
#[test]
fn test_sign_duration_from_micros() {
    let script = make_script("duration := SignedDuration.from_micros(1000)\nwritestdout(duration.as_millis())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1.0"));
    assert!(output.status.success());

}
#[test]
fn test_sign_duration_from_nanos() {
    let script = make_script("duration := SignedDuration.from_nanos(1000000)\nwritestdout(duration.as_millis())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1.0"));
    assert!(output.status.success());
}

#[test]
fn test_signed_duration_from_hours() {
    let script = make_script("duration := SignedDuration.from_hours(1)\nwritestdout(duration.as_secs())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("3600.0"));
    assert!(output.status.success());
}

#[test]
fn test_signed_duration_is_zero() {
    let script = make_script("duration := SignedDuration.ZERO()\nwritestdout(duration.is_zero())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1"));
    assert!(output.status.success());
}

#[test]
fn test_signed_duration_is_positive() {
    let script = make_script("duration := SignedDuration.MAX()\nwritestdout(duration.is_positive())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1"));
    assert!(output.status.success());
}


#[test]
fn test_signed_duration_is_negative() {
    let script = make_script("duration := SignedDuration.MIN()\nwritestdout(duration.is_negative())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1"));
    assert!(output.status.success());
}


#[test]
fn test_signed_duration_checked_neg() {
    let script = make_script("duration := SignedDuration.MAX()\nwritestdout(duration.checked_neg().is_negative())");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1"));
    assert!(output.status.success());
}


#[test]
fn test_signed_duration_checked_add() {
    let script = make_script("duration1 := SignedDuration.new(12, 500000000)\nduration2 := SignedDuration.new(0, 500000000)\nexpected := SignedDuration.new(13, 0)\nwritestdout(duration1.checked_add(duration2).eq(expected))");
    let output = run_script(script);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "");
    assert_eq!(stdout.to_string(), String::from("1"));
    assert!(output.status.success());
}