mod lib;
use lib::save_screenshot;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();

    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, r#"{}{}ctrl + n to create a screenshot{}ctrl + q to exit"#, termion::cursor::Goto(1, 1), termion::clear::All, termion::cursor::Goto(1, 2),)
            .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        match c.unwrap() {
            Key::Ctrl('n') => save_screenshot(&mut stdout),
            Key::Ctrl('q') => return,
            _ => (),
        }

        stdout.flush().unwrap();
    }
}