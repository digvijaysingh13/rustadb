use std::process::Command;

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
