use std::path::PathBuf;
use std::process::Command;
use std::{io};

pub fn run_adb_cmd(args: Vec<String>) {
    let mut cmd = Command::new("adb");
    for each in args {
        cmd.arg(each);
    }
    let out = cmd.output();
    if let Err(e) = &out {
        println!("{}", e);
    } else {
        let out = out.unwrap();
        println!("status: {}", out.status);
        let error = out.stderr;
        if error.len() != 0 {
            let error = String::from_utf8_lossy(&error);
            println!("{}", error);
        }
        let output = out.stdout;
        if output.len() != 0 {
            let output = String::from_utf8_lossy(&output);
            println!("{}", output);
        }
    }
}

pub fn get_adb_output(args: Vec<&str>) -> Result<Vec<u8>, io::Error> {
    return get_cmd_output("adb", args);
}

pub fn get_cmd_output(cmd: &str, args: Vec<&str>) -> Result<Vec<u8>, io::Error> {
    let mut cmd = Command::new(cmd);
    for each in args {
        cmd.arg(each);
    }
    let out = cmd.output()?;
    println!("status: {}", out.status);
    let output = out.stdout;
    return Ok(output);
}

pub fn get_time_str() -> String {
    let sys_time = chrono::offset::Local::now();
    return format!("{}", sys_time);
}

/**
 * Checks if radb_output directory exists or not at desktop
 * if not then create it.
 */
pub fn get_output_dir() -> Option<PathBuf> {
    let desktop = dirs::desktop_dir();
    if let Some(path_buff) = desktop {
        let desktop_path = path_buff.join("radb_output");
        if !desktop_path.exists() {
            // create directory
            let result = std::fs::create_dir(&desktop_path);
            if let Err(error) = result {
                println!("{}", error);
                return None;
            }
        }
        return Some(desktop_path);
    }
    return None;
}
