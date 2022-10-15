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
const INSTALL_TOOLS: &str = "installtool";
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
    println!("{}: to show the all commands and their used.", SHOW_INFO);
    println!();

    // Exit command
    println!("{}: to close radb app.", EXIT);
    println!();
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
    println!("Execcuting the command.");
}
