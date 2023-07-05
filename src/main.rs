use regex::Regex;
use tokio::process::Command;
#[tokio::main]
async fn main() {
    let  mut command = Command::new("powershell");
    command.args(&["-Command", "chcp 65001 && ver"]);
    let output = command.output().await.unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let regex = Regex::new(r"\S*.\]").unwrap();
    let mut output = regex.find(&stdout).unwrap().as_str().to_string();
    output.pop();
    println!("{}", output);
}
