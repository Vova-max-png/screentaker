use screenshots::Screen;
use chrono::Local;
use std::io::Write;
use whoami::Platform;
use std::{thread, time};

fn delay(millis: u64, stream: &mut termion::raw::RawTerminal<std::io::Stdout>) {
    thread::sleep(time::Duration::from_millis(1000));
    printcln(format!("{}{}Waiting for {} millis...", termion::cursor::Goto(1, 1), termion::clear::All, millis), stream);
    thread::sleep(time::Duration::from_millis(millis));
}

fn printcln(data: String, stream: &mut termion::raw::RawTerminal<std::io::Stdout>) {
    write!(stream, "{}", data)
         .unwrap();
    stream.flush().unwrap();
}

fn take_screenshot(path: String) -> String {
    let start_time = Local::now();
    Screen::from_point(0, 0)
        .unwrap()
        .capture_area(0, 0, 1920, 1080)
        .unwrap()
        .save(format!("{}{}.png", path, start_time.clone()))
        .unwrap();
    format!("{}{}.png", path, start_time)
}

pub fn save_screenshot(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
    let user_name = whoami::realname();
    let path = match whoami::platform() {
        Platform::Linux => {
            printcln(format!("{}{}You are on Linux, saving to '/home/{}/Documents/'", termion::cursor::Goto(1, 1), termion::clear::All, user_name), stdout);
            format!("/home/{}/Documents/", user_name)
        },
        Platform::Windows => {
            printcln(format!("{}{}You are on Windows, saving to 'C:/Users/{}/Documents/'", termion::cursor::Goto(1, 1), termion::clear::All, user_name), stdout);
            format!("C:/Users/{}/Documents/", user_name)
        },
        Platform::MacOS => {
            printcln(format!("{}{}You are on MacOS, saving to '/Users/{}/Documents/'", termion::cursor::Goto(1, 1), termion::clear::All, user_name), stdout);
            format!("/Users/{}/Documents/", user_name)
        },
        _ => {"".to_string()}
    };
    delay(3000, stdout);
    let taken_path = take_screenshot(path.clone());
    printcln(format!("{}{}Screenshot saved as '{}'", termion::cursor::Goto(1, 1), termion::clear::All, taken_path), stdout)
}