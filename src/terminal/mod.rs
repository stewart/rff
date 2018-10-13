mod event;
mod input;
use libc::{cfmakeraw, ioctl};
use libc::{tcgetattr, tcsetattr, termios};
use libc::{winsize, TCSANOW, TIOCGWINSZ};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::mem;
use std::os::unix::io::AsRawFd;

pub use self::event::*;
pub use self::input::*;

#[derive(Debug)]
pub enum Error {
    TcGetAttr,
    TcSetAttr,
}

pub struct Terminal {
    file: File,
    prev_termios: Option<termios>,
    pub max_width: usize,
    pub max_height: usize,
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
            max_height: 25,
        };

        unsafe {
            let mut ws: winsize = mem::zeroed();
            if ioctl(fd, TIOCGWINSZ, &mut ws) != -1 {
                terminal.max_width = ws.ws_col as usize;
                terminal.max_height = ws.ws_row as usize;
            }
        }

        Ok(terminal)
    }

    pub fn set_raw_mode(&mut self) -> Result<(), Error> {
        let fd = self.file.as_raw_fd();

        unsafe {
            let mut ios: termios = mem::zeroed();

            // get the existing termios
            if tcgetattr(fd, &mut ios) != 0 {
                return Err(Error::TcGetAttr);
            }

            self.prev_termios = Some(ios);

            // enable raw mode
            cfmakeraw(&mut ios);

            // apply the raw mode termios
            if tcsetattr(fd, TCSANOW, &ios) != 0 {
                return Err(Error::TcSetAttr);
            }
        }

        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), Error> {
        if let Some(opts) = self.prev_termios {
            let fd = self.file.as_raw_fd();

            // disable raw mode, by setting the original termios
            unsafe {
                if tcsetattr(fd, TCSANOW, &opts) != 0 {
                    return Err(Error::TcSetAttr);
                }
            }
        }

        Ok(())
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
        self.reset().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

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
