use crate::ui::UI;
use screenshots::Screen;
use chrono::Local;
use whoami::Platform;
use std::{thread, time};

pub struct Screenshot<'a> {
    ui: &'a mut UI<'a>,
}

impl<'c> Screenshot<'c> {
    pub fn new(ui: &'c mut UI<'c>) -> Self {
        Self {
            ui,
        }
    }

    pub fn save_screenshot(&mut self) {
        self.ui.clear();
        let user_name = whoami::realname();
        let path = match whoami::platform() {
            Platform::Linux => {
                self.ui.print_only(format!("You are on Linux, saving to '/home/{}/Documents/'", user_name).as_str());
                format!("/home/{}/Documents/", user_name)
            },
            Platform::Windows => {
                self.ui.print_only(format!("You are on Windows, saving to 'C:/Users/{}/Documents/'", user_name).as_str());
                format!("C:/Users/{}/Documents/", user_name)
            },
            Platform::MacOS => {
                self.ui.print_only(format!("You are on MacOS, saving to '/Users/{}/Documents/'", user_name).as_str());
                format!("/Users/{}/Documents/", user_name)
            },
            _ => {"".to_string()}
        };
        self.delay(3000);
        let taken_path = self.take_screenshot(path.clone());
        self.ui.print_only(format!("Screenshot saved as '{}'", taken_path).as_str());
    }

    fn delay(&mut self, millis: u64) {
        let secs = millis/1000;
        for i in 0..=secs {
            thread::sleep(time::Duration::from_millis(1000));
            self.ui.print_only(format!("Waiting for {} seconds...", secs-i).as_str());
        }
    }
    
    fn take_screenshot(&self, path: String) -> String {
        let start_time = Local::now();
        Screen::from_point(0, 0)
            .unwrap()
            .capture_area(0, 0, 1920, 1080)
            .unwrap()
            .save(format!("{}{}.png", path, start_time.clone()))
            .unwrap();
        format!("{}{}.png", path, start_time)
    }
}