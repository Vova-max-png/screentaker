mod screenshot;
mod ui;
use screenshot::Screenshot;
use ui::UI;
use std::io::{stdin, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();

    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut ui = UI::new(&mut stdout);

    ui.print_only("ctrl + n to create a screenshot");
    ui.println("ctrl + q to exit");

    let mut screenshot = Screenshot::new(&mut ui);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('n') => screenshot.save_screenshot(),
            Key::Ctrl('q') => return,
            _ => (),
        }
    }
}