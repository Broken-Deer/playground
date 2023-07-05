// use regex::Regex;
// use regex::Regex;
// use tokio::{io::AsyncBufReadExt, process::Command};
// #[tokio::main]
// async fn main() {
//     let mut command = Command::new("C:\\Windows\\System32\\cmd.exe");
//     command.args(&["/C", r#"powershell -c [System.Environment]::OSVersion.Version"#]);
//     let output = command.output().await.unwrap();
//     let stdout = String::from_utf8(output.stdout).unwrap();

//     let regex = Regex::new(r"\s+").unwrap();
//     let output = regex.replace_all(&stdout, ".").to_string();
//     println!("{}", output);
// }

use mgl_core::core::{folder::MinecraftLocation, task::EventListeners};

#[tokio::main]
async fn main() {
    // let listeners = EventListeners::new().on_progress(Box::new(|a, b, c| {
    //     println!("progress: {a}/{b}  step: {c}");
    // }));
    // mgl_core::installer::install("1.19.4", MinecraftLocation::new("test"), listeners).await;
    println!("Hello, world!");
}
