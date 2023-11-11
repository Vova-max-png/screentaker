use std::io::Write;

pub struct UI<'a> {
    stream: &'a mut termion::raw::RawTerminal<std::io::Stdout>,
}

impl<'c> UI<'c> {
    pub fn new(stream:  &'c mut termion::raw::RawTerminal<std::io::Stdout>) -> Self {
        Self {
            stream,
        }
    }

    #[allow(unused)]
    pub fn print(&mut self, data: &str) {
        write!(self.stream, "{}", data)
            .unwrap();
        self.flush();
    }

    #[allow(unused)]
    pub fn println(&mut self, data: &str) {
        write!(self.stream, "{}{}", "\r\n", data)
            .unwrap();
        self.flush();
    }

    #[allow(unused)]
    pub fn print_only(&mut self, data: &str) {
        write!(self.stream, "{}{}{}", termion::cursor::Goto(1, 1), termion::clear::All, data)
            .unwrap();
        self.flush();
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        write!(self.stream, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All);
        self.flush();
    }

    fn flush(&mut self) {
        self.stream.flush().unwrap();
    }
}