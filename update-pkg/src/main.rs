use std::error::Error;

use std::process::Command;
use std::process::Output;

pub fn execute_shell_cmd(cmd: &str) -> Result<Output, Box<dyn Error>> {
    Ok(Command::new("sh").arg("-c").arg(cmd).output()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = "apt list --upgradable";
    let mut output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines = stdout.lines();
    let mut cmd_buffer: String = String::from("sudo apt install -s -y ");
    for line in lines {
        if let Some(pos) = line.find('/') {
            let value = &line[..pos];
            cmd_buffer.push_str(&(value.to_owned() + " "));
        }
        //packages.push(line.to_string());
    }
    println!("{}", cmd_buffer);

    output = execute_shell_cmd(&cmd_buffer)?;

    println!("{:#?}", output);
    Ok(())
}
