use tokio::process::Command;


// use regex::Regex;
// use regex::Regex;
// use tokio::{io::AsyncBufReadExt, process::Command};
#[tokio::main]
async fn main() {
    let mut command = Command::new("C:\\Windows\\System32\\cmd.exe");
    command.args(&["/C", r#"powershell -c [System.Environment]::OSVersion.Version"#]);
    let output = command.output().await.unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    println!("{}", &stdout);
    println!("{}", &stderr);
    // let a = r"
    // OsHardwareAbstractionLayer
    // --------------------------
    // 10.0.19041.2251

    // Microsoft Windows [version 10.0]";
    // let regex = Regex::new(r"[0-9].*\S").unwrap();
    // let output = regex.find(&a).unwrap().as_str().to_string();

    // println!("{}", output);
}
