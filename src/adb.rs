use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

use crate::utils::{get_adb_output, get_output_dir, get_time_str, run_adb_cmd};

// declaration of commands
const SHOW_INFO: &str = "showinfo";

/**
 * ADB basic commands
*/
const START_SERVER: &str = "start";
const STOP_SERVER: &str = "stop";
const LIST_DEVICES: &str = "ls";
const SHOW_IP: &str = "ip";
const CONNECT_WITH_WIFI: &str = "connect";
const LOGCAT: &str = "logcat";

/**
 * adb install and set-up commands
 */
const DOWNLOAD_TOOLS: &str = "download";
const SET_PATH: &str = "setpath";

/**
 * adb phone interaction related commands
*/
const SCREEN_CAPTURE: &str = "capture";
const SCREEN_RECORD: &str = "record";
const DUMP: &str = "dump";
const LIST_PACKAGE: &str = "listpackage";
const PULL_APK: &str = "pull";
const INSTALL_APK: &str = "install";

/**
 * adb exit commands
 */
const EXIT: &str = "exit";

fn show_info() {
    println!(
        "{:015}: to show the all commands and their used.",
        SHOW_INFO
    );
    println!();

    // description printing for basic adb commands
    println!(
        "{:015}: to start adb server. This is equivalent to adb start-server.",
        START_SERVER
    );
    println!(
        "{:015}: to stop adb server. This is equivalent to adb kill-server.",
        STOP_SERVER
    );
    println!(
        "{:015}: to show the list of connected devices. This is equivalent to adb devices.",
        LIST_DEVICES
    );
    println!("{:015}: to show ip of connected device.", SHOW_IP);
    println!("{:015}: to connect the device with wifi.\n\te.g. {} IP:PORT.\n\tYou can get the IP of phone in Settings > Wifi Settings > Advance > IP Address.\n\tOr Search IP address in Setting. Or Use ip command.\n\tMake sure phone is connect through USB.",CONNECT_WITH_WIFI, CONNECT_WITH_WIFI);
    println!("{:015}: to record logcat in file, after this command add file name where logcat should be stored.\n\teg. logcat demo.", LOGCAT);
    println!();
    // description printing of install and setup commands
    println!(
        "{:015}: to download android command line tools and platform tools in home dir.",
        DOWNLOAD_TOOLS
    );
    println!(
        "{:015}: to set the downloaded tools enviromental path.",
        SET_PATH
    );
    println!();
    // description printing of phone interaction related commands
    println!(
        "{:015}: to capture the screen of connected devices.\n\teg. {} filename.",
        SCREEN_CAPTURE, SCREEN_CAPTURE
    );
    println!(
        "{:015}: to record the screen of connected devices.\n\teg. {} filename.",
        SCREEN_RECORD, SCREEN_RECORD
    );
    println!(
        "{:015}: to get dump of system or application.\n\teg. {} packagename.",
        DUMP, DUMP
    );
    println!(
        "{:015}: to print all installed applications packagename.",
        LIST_PACKAGE
    );
    println!(
        "{:015}: to pull the installed application from phone to computer.\n\teg. {} packagename.",
        PULL_APK, PULL_APK
    );
    println!(
        "{:015}: to install apk in connected phone.\n\teg. {} location/of/your/apk.",
        INSTALL_APK, INSTALL_APK
    );
    println!();
    // Exit command
    println!("{}: to close radb app.", EXIT);
    println!("\n");
}

fn get_cmd() -> Option<Vec<String>> {
    let mut input_string = String::new();
    let read = std::io::stdin().read_line(&mut input_string).ok();
    if let Some(_) = read {
        // removing spacing chars
        input_string = input_string.trim().to_owned();
        // removing new line \n chars
        if input_string.ends_with('\n') {
            input_string.pop();
        }
        // converting string into vec string
        let result: Vec<String> = input_string.split(" ").map(|p| p.to_owned()).collect();
        return Some(result);
    }
    return None;
}

pub fn run_adb() {
    println!("============radb started=============");
    println!();
    show_info();
    set_adb_path();
    loop {
        println!("Enter the command$");
        let cmd = get_cmd();
        match cmd {
            Some(cmd) => {
                if cmd.len() == 0 || cmd[0].len() == 0 {
                    println!("No command found. Enter the command.");
                    continue;
                }
                if cmd[0] == "exit" {
                    println!("==========Bye bye=========");
                    break;
                }
                exe_cmd(cmd);
            }
            None => {
                println!("Some error occured!!!, no command found.");
            }
        }
    }
}

fn exe_cmd(cmd: Vec<String>) {
    let c: &str = cmd[0].as_str();
    if c == DOWNLOAD_TOOLS {
        // to download and setting tools
        download_adb_tool();
    } else if c == SET_PATH {
        // to set path
        set_adb_path();
    } else if c == START_SERVER {
        // to start adb server
        run_adb_cmd(vec!["start-server".to_owned()]);
    } else if c == STOP_SERVER {
        // to stop adb server
        run_adb_cmd(vec!["kill-server".to_owned()]);
    } else if c == LIST_DEVICES {
        // to list connected devices
        run_adb_cmd(vec!["devices".to_owned()]);
    } else if c == SHOW_IP {
        // to show ip of connected phone
        run_adb_cmd(vec![
            "shell".to_owned(),
            "ip".to_owned(),
            "addr".to_owned(),
            "show".to_owned(),
            "wlan0".to_owned(),
        ]);
    } else if c == LOGCAT {
        if cmd.len() < 2 {
            adb_logcat(None);
        } else {
            let fname = &cmd[1];
            adb_logcat(Some(&fname))
        }
    } else if c == SHOW_INFO {
        show_info();
    } else if c == CONNECT_WITH_WIFI {
        // to connect the adb server on wifi
        // checkout the validation
        if cmd.len() != 2 {
            println!("Too many or too less args provided. do like eg. connect 192.198.0.1:5555");
            return;
        }
        let mut ip_ports = cmd[1].to_owned();
        let arr: Vec<String> = ip_ports.split(":").map(|e| e.to_owned()).collect();
        let port = if arr.len() != 2 {
            println!("port not entered, adding default port 5555.");
            ip_ports.push_str(":5555");
            "5555"
        } else {
            &arr[1]
        };
        run_adb_cmd(vec!["tcpip".to_owned(), port.to_owned()]);
        run_adb_cmd(vec!["connect".to_owned(), ip_ports.to_owned()]);
    } else if c == SCREEN_CAPTURE {
        capture_screenshot(&cmd);
    } else if c == SCREEN_RECORD {
        if cmd.len() < 2 {
            println!("insufficient arguments supplied. eg. record nameOfVideoFileYouWant.");
            return;
        }
        record_video(&cmd[1]);
    } else if c == DUMP {
        if cmd.len() < 2 {
            println!("insufficient arguments provided. eg. dump com.your.packagename");
        }
        dump_sys(&cmd[1]);
    } else if c == INSTALL_APK {
        // checkout the if path arg is entered or not
        if cmd.len() < 2 {
            println!("Apk location is not provided.");
            return;
        }
        let location = &cmd[1];
        run_adb_cmd(vec![
            "install".to_owned(),
            "-r".to_owned(),
            location.to_owned(),
        ]);
    } else if c == PULL_APK {
        // checkout the if path arg is entered or not
        if cmd.len() < 2 {
            println!("package name not provided.");
            return;
        }
        let package_name = &cmd[1];
        pull_apk(&package_name);
    } else if c == LIST_PACKAGE {
        run_adb_cmd(vec![
            "shell".to_owned(),
            "pm".to_owned(),
            "list".to_owned(),
            "packages".to_owned(),
        ])
    }
}

fn download_adb_tool() {
    // check if adb_cmdtools exits, if it exits then do nothing.
    let adb_path = check_or_create_tools_dir();
    if let Some(path) = adb_path {
        // changing current working dir
        let result = std::env::set_current_dir(&path);
        if let Err(err) = result {
            println!("{}", err);
            return;
        }
        // clone the zip files from github.
        let result = clone_adb_files();
        if let Err(err) = result {
            println!("{}", err);
            return;
        }
        // extract and unzip the files.
        let result = extracting_adb_files(&path);
        if let Err(err) = result {
            println!("{}", err);
            return;
        }
        // at last set the path
        set_adb_path();
    }
}

fn check_or_create_tools_dir() -> Option<PathBuf> {
    let home_path = dirs::home_dir();
    if let None = home_path {
        println!("Could not find home dir.");
        return None;
    }
    let mut home_path = home_path.unwrap();
    // create new adb_cmdtools dir.
    home_path = home_path.join("adb_cmdtools");
    if home_path.exists() {
        // in case if adb tools dir already exists, then ask user wheather to overwrite or not.
        // get input from console, if user enters y?Y then delete the existing dirs
        // rest proceeds same.
        println!("adb tools already exits. Do you want to overwrite?. Enter (y/Y).");
        let c = get_cmd();
        if let None = c {
            return None;
        }
        let op = c.unwrap();
        if op.len() > 0 || op[0].to_lowercase() == "y" {
            println!("deleting existing setup.");
            let result = std::fs::remove_dir_all(&home_path);
            if let Err(e) = result {
                println!("{}", e);
                return None;
            }
        } else {
            // return in else case
            return None;
        }
    }
    let result = std::fs::create_dir(&home_path);
    // if can not create path then print error and return
    if let Err(e) = result {
        println!("{}", e);
        return None;
    }
    return Some(home_path);
}

const ZIPPED_ADB_URL: &str = "https://github.com/digvijaysingh13/adb-cmdtools-linux.git";

fn clone_adb_files() -> Result<(), io::Error> {
    println!("Cloning the tools...");
    let mut cmd = Command::new("git");
    cmd.arg("clone");
    cmd.arg(ZIPPED_ADB_URL);
    cmd.output()?;
    return Ok(());
}

fn extracting_adb_files(path: &PathBuf) -> Result<(), io::Error> {
    println!("unzipping...");
    // moving internal files one level up
    let working_path = path.clone().into_os_string().into_string().unwrap();
    // moving platform tools to one level
    let platform_path = path
        .join("adb-cmdtools-linux")
        .join("platform-tools.zip")
        .into_os_string()
        .into_string()
        .unwrap();
    let mut platform_cmd = Command::new("mv");
    platform_cmd.arg(platform_path);
    platform_cmd.arg(&working_path);
    platform_cmd.output()?;
    // moving command line tools to one level
    let cmdtools_path = path
        .join("adb-cmdtools-linux")
        .join("cmdline-tools.zip")
        .into_os_string()
        .into_string()
        .unwrap();
    let mut cmd = Command::new("mv");
    cmd.arg(cmdtools_path);
    cmd.arg(&working_path);
    cmd.output()?;
    // remove the cloned dir with readme file
    fs::remove_dir_all(path.join("adb-cmdtools-linux"))?;
    // unzip the cmdline-tools and platform-tools
    // unziping cmdline-tools
    let mut cmd = Command::new("unzip");
    cmd.arg("cmdline-tools.zip");
    cmd.output()?;
    // unzipping platform-tools
    let mut cmd = Command::new("unzip");
    cmd.arg("platform-tools.zip");
    cmd.output()?;
    // remove the unzip files
    let mut cmd = Command::new("rm");
    cmd.arg("cmdline-tools.zip");
    cmd.arg("platform-tools.zip");
    cmd.output()?;
    return Ok(());
}

fn set_adb_path() {
    // check if adb_cmdtools folder exist
    let home = dirs::home_dir();
    if let Some(path) = home {
        let adbpath = path.join("adb_cmdtools");
        if !adbpath.exists() {
            println!("tools is download. use {} to download it.", DOWNLOAD_TOOLS);
            return;
        }
        let path_of_cmdline = adbpath
            .join("cmdline-tools")
            .join("bin")
            .into_os_string()
            .into_string()
            .unwrap();
        let path_of_platform = adbpath
            .join("platform-tools")
            .into_os_string()
            .into_string()
            .unwrap();
        let result = std::env::var("PATH");
        match result {
            Ok(path) => {
                let path = format!("{}:{}:{}", path, path_of_cmdline, path_of_platform);
                std::env::set_var("PATH", path);
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    } else {
        println!("home dir not found!!!");
    }
}

fn capture_screenshot(cmds: &Vec<String>) {
    // to capture the screen shot;
    let output_dir = get_output_dir();
    if let Some(output_path) = output_dir {
        let mut filename = String::new();
        // check if user added the file name
        if cmds.len() > 1 {
            filename.push_str(&cmds[1]);
        }
        let stamp = get_time_str();
        filename.push_str(&stamp);
        // adding extention name
        filename.push_str(".png");
        // create file
        let filename = output_path.join(filename);
        let file = fs::File::create(filename);
        if let Err(error) = &file {
            println!("{}", error);
            return;
        }
        let mut file = file.unwrap();
        // get data for command
        let data = get_adb_output(vec!["shell", "screencap", "-p"]);
        if let Err(error) = &data {
            println!("{}", error);
            return;
        }
        let data = data.unwrap();
        let result = file.write_all(&data);
        if let Err(error) = result {
            println!("{}", error);
            return;
        }
        // printing final success message
        println!("capturing screen is done. checkout output in radb_output dir at desktop.")
    } else {
        println!("No output dir is found.");
    }
}

fn dump_sys(package_name: &str) {
    let mut sysdump_cmd = Command::new("adb");
    sysdump_cmd.arg("shell");
    sysdump_cmd.arg("dumpsys");
    sysdump_cmd.arg("activity");

    let child = sysdump_cmd.stdout(std::process::Stdio::piped()).spawn();
    if let Err(err) = &child {
        println!("{}", err);
    }
    let child = child.unwrap();

    let mut grep_cmd = Command::new("grep");
    grep_cmd.arg("-i");
    grep_cmd.arg(package_name);
    grep_cmd.stdin(child.stdout.unwrap());

    let out_result = grep_cmd.output();
    if let Ok(out) = out_result {
        let output = out.stdout;
        let out = String::from_utf8_lossy(&output);
        println!("{}", out);
        // save the file
        let adb_dir = get_output_dir();
        if let None = adb_dir {
            // just print the whatever output
            println!("{}", &out);
            return;
        }
        let dumpfile = adb_dir.unwrap().join("dumpfile.txt");
        if dumpfile.exists() {
            let result = fs::remove_file(&dumpfile);
            if let Err(error) = result {
                println!("{}", error);
            }
        }
        // create new file
        let file = File::create(dumpfile);
        if let Err(error) = &file {
            println!("{}", error);
            return;
        }
        let mut file = file.unwrap();
        let result = file.write_all(&output);
        if let Err(error) = &result {
            println!("{}", error);
            return;
        } else {
            println!("\n\n\ndumpfile.txt is stored in radboutput dir on desktop.\\n\n");
        }
    } else {
        println!("{}", out_result.err().unwrap());
    }
}

fn pull_apk(package_name: &str) {
    let output_dir = get_output_dir().unwrap();
    let output_dir = output_dir.into_os_string().into_string().unwrap();
    let package_path = get_adb_output(vec!["shell", "pm", "path", package_name]);
    if let Err(error) = package_path {
        println!("{}", error);
        return;
    }
    let package_path = String::from_utf8(package_path.unwrap()).unwrap();
    let pull_package_path = &package_path.trim()[8..];
    println!("pull apk from {} to {}.", pull_package_path, &output_dir);
    run_adb_cmd(vec![
        "pull".to_owned(),
        pull_package_path.to_owned(),
        output_dir,
    ])
}

fn adb_logcat(save_path: Option<&str>) {
    // first clear the exiting logcat
    run_adb_cmd(vec![
        "logcat".to_owned(),
        "-b".to_owned(),
        "all".to_owned(),
        "-c".to_owned(),
    ]);
    println!("out adb buffer cleared.");
    if save_path.is_none() {
        run_adb_cmd(vec!["logcat".to_owned()]);
        return;
    }
    // now create the file with same name
    let home_path = get_output_dir().unwrap();
    let mut logcat_file_name = String::new();
    logcat_file_name.push_str(save_path.unwrap());
    let time = get_time_str();
    let time = time.replace(":", "");
    let time = time.replace(".", "");
    let time = time.replace(" ", "");
    let time = time.replace('+', "");
    let time = format!("_{}.txt", time);
    logcat_file_name.push_str(&time);
    let logcat_file_path = home_path.join(logcat_file_name);
    // if something like this exists then delete it
    if logcat_file_path.exists() {
        if let Err(error) = fs::remove_file(&logcat_file_path) {
            println!("{}", error);
            return;
        }
    }

    // create the file
    let file = fs::File::create(&logcat_file_path);
    if let Err(error) = &file {
        println!("{}", error);
        return;
    }

    let file = file.unwrap();
    let stdio = std::process::Stdio::from(file);
    let mut logcat_cmd = Command::new("adb");
    logcat_cmd.arg("logcat");
    logcat_cmd.stdout(stdio);
    let child = logcat_cmd.spawn();
    if let Err(error) = child {
        println!("{}", error);
        return;
    }
    let mut child = child.unwrap();
    loop {
        println!("Enter quit to stop logcat commanding");
        let cmd = get_cmd();
        if let Some(cmd) = cmd {
            if cmd.len() < 1 {
                continue;
            }
            let c = &cmd[0];
            if c == "quit" {
                if let Err(error) = child.kill() {
                    println!("{}", error);
                }
                break;
            }
        }
    }
}

fn record_video(video_name: &str) {
    // start video recording in phone storage
    let mut video_cmd = Command::new("adb");
    video_cmd.arg("shell");
    video_cmd.arg("screenrecord");
    video_cmd.arg("/sdcard/radbvideo.mp4");
    let child = video_cmd.spawn();
    if let Err(error) = &child {
        println!("{}", error);
        return;
    }
    let mut child = child.unwrap();
    loop {
        println!("Enter stop to stop recording of video");
        let cm = get_cmd();
        if cm.is_none() {
            continue;
        }
        let cm = cm.unwrap();
        if cm.len() < 1 {
            continue;
        }
        if "stop" == &cm[0] {
            let r = child.kill();
            if let Err(error) = r {
                println!("{}", error);
                continue;
            }
            break;
        }
    }
    // after video recording is finished pull the apk
    // create unique file
    let home = get_output_dir();
    if home.is_none() {
        delete_recorded_video();
        return;
    }
    let file_path = home.unwrap().join(format!("{}.mp4", video_name));
    if file_path.exists() {
        // delete the existing file.
        println!("deleting the existing file ...");
        fs::remove_file(&file_path).unwrap();
    }
    let file = fs::File::create(&file_path);
    if let Err(err) = file {
        println!("{}", err);
        delete_recorded_video();
        return;
    }
    println!("Pulling video...");
    // after file create pull the video
    let dest_path = file_path.into_os_string().into_string().unwrap();
    // wait for some time 5 secs
    std::thread::sleep(
        std::time::Duration::from_secs(5)
    );
    run_adb_cmd(vec![
        "pull".to_owned(),
        "/sdcard/radbvideo.mp4".to_owned(),
        dest_path,
    ]);
    // after pulling is completed delete the video from sdcard
    delete_recorded_video();
}

fn delete_recorded_video() {
    run_adb_cmd(vec![
        "shell".to_owned(),
        "rm".to_owned(),
        "/sdcard/radbvideo.mp4".to_owned(),
    ]);
}
