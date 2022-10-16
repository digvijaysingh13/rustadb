use crate::utils::get_home_dir;

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
    println!("{:015}: to record logcat in file, after this command add file name where logcat should be stored.\n\teg. logcat demo.txt.", LOGCAT);
    println!();
    // description printing of install and setup commands
    println!("{:015}: to download android command line tools and platform tools in home dir.", DOWNLOAD_TOOLS);
    println!("{:015}: to set the downloaded tools enviromental path.", SET_PATH);
    println!();
    // description printing of phone interaction related commands
    println!("{:015}: to capture the screen of connected devices.\n\teg. {} filename.png.", SCREEN_CAPTURE, SCREEN_CAPTURE);
    println!("{:015}: to record the screen of connected devices.\n\teg. {} filename.mp4.", SCREEN_RECORD, SCREEN_RECORD);
    println!("{:015}: to get dump of system or application.\n\teg. {} packagename.", DUMP, DUMP);
    println!("{:015}: to print all installed applications packagename.", LIST_PACKAGE);
    println!("{:015}: to pull the installed application from phone to computer.\n\teg. {} packagename.", PULL_APK, PULL_APK);
    println!("{:015}: to install apk in connected phone.\n\teg. {} location/of/your/apk.", INSTALL_APK, INSTALL_APK);
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
        let home = get_home_dir();
        match home {
            Ok(path) => {
                println!("{}", path);
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}
