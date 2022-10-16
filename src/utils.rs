use std::{
    io::Error,
    process::{Command, Output},
};

pub fn get_home_dir() -> Result<String, String> {
    let mut home = Command::new("echo");
    // home.arg("$HOME");
    let out: Result<Output, Error> = home.output();
    match out {
        Ok(data) => {
            let d = String::from_utf8(data.stdout);
            if let Ok(path) = d {
                return Ok(path);
            }
            return Err("Could not read file path characters.".to_owned());
        }
        Err(io_err) => {
            return Err(io_err.to_string());
        }
    }
}
