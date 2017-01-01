mod event;
mod input;
use std::mem;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::os::unix::io::AsRawFd;
use libc::{TCSANOW, TIOCGWINSZ, termios, winsize, c_int, c_ulong};

pub use self::input::*;
pub use self::event::*;

extern {
    fn tcgetattr(filedes: c_int, termptr: *mut termios) -> c_int;
    fn tcsetattr(filedes: c_int, opt: c_int, termptr: *const termios) -> c_int;
    fn cfmakeraw(termptr: *mut termios);
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

#[derive(Debug)]
pub enum Error {
    TcGetAttr,
    TcSetAttr
}

pub struct Terminal {
    file: File,
    prev_termios: Option<termios>,
    pub max_width: u16,
    pub max_height: u16,
}

impl Terminal {
    /// Creates a new Terminal from the provided filename
    pub fn from(filename: &str) -> io::Result<Terminal> {
        let file = OpenOptions::new().write(true).read(true).open(filename)?;
        let fd = file.as_raw_fd();

        let mut terminal = Terminal {
            file: file,
            prev_termios: None,
            max_width: 80,
            max_height: 25
        };

        unsafe {
            let mut ws: winsize = mem::zeroed();
            if ioctl(fd, TIOCGWINSZ, &mut ws) != -1 {
                terminal.max_width = ws.ws_col;
                terminal.max_height = ws.ws_row;
            }
        }

        Ok(terminal)
    }

    pub fn set_raw_mode(&mut self) -> Result<(), Error> {
        let fd = self.file.as_raw_fd();

        unsafe {
            let mut ios: termios = mem::zeroed();

            // get the existing termios
            let result = tcgetattr(fd, &mut ios);
            if result != 0 {
                return Err(Error::TcGetAttr);
            }

            self.prev_termios = Some(ios);

            // enable raw mode
            cfmakeraw(&mut ios);

            // apply the raw mode termios
            let result = tcsetattr(fd, TCSANOW, &ios);
            if result != 0 {
                return Err(Error::TcSetAttr);
            }
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        if let Some(opts) = self.prev_termios {
            let fd = self.file.as_raw_fd();

            // disable raw mode, by setting the original termios
            unsafe { tcsetattr(fd, TCSANOW, &opts); }
        }
    }

    pub fn events(&self) -> io::Result<Events<File>> {
        self.file.try_clone().map(Events::new)
    }
}

impl Read for Terminal {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read(buf)
    }
}

impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal() {
        let mut term = Terminal::from("/dev/tty").expect("Unable to open /dev/tty");

        // if all is good, this should _not_ break the terminal, because the
        // Drop trait impl will clean up
        term.set_raw_mode().expect("Unable to enable raw mode");

        write!(term, "").unwrap();
        term.flush().unwrap();
    }
}
