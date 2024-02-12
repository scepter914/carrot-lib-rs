use log::info;
use std::process::Command;

/// Get command output.
/// # Arguments
/// - command: The input command
pub fn get_command_output(command: &str) -> String {
    let args_list = ["-c", command];
    let command_output = Command::new("sh")
        .args(&args_list)
        .output()
        .expect("failed command");
    String::from_utf8_lossy(&command_output.stdout).to_string()
}

/// Print command result.
/// # Arguments
/// - command: The input command
/// - command_output: The output of command
pub fn print_command_result(command: &str, command_output: &str) {
    info!("command: {}\n\n{}", command, command_output);
}
