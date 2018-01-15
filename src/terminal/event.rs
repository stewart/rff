// A subset of termion's event parsing.
// Termion (c) 2016 Ticki
// Licensed under the MIT license

use std::io::{Result, Error, ErrorKind};
use std::str;

#[allow(unused_imports)] use std::ascii::AsciiExt;

#[derive(Debug)]
pub enum Event {
    Key(Key),
    Unknown(Vec<u8>)
}

#[derive(Debug)]
pub enum Key {
    Escape,
    Backspace,
    Tab,

    Left, Right, Up, Down,

    Home, End,

    PageUp, PageDown,

    Delete, Insert,

    Char(char),
    Alt(char),
    Ctrl(char),

    F(u8),

    Null
}

/// Parse an Event from `item` and possibly subsequent bytes through `iter`.
pub fn parse_event<I>(item: u8, iter: &mut I) -> Result<Event>
    where I: Iterator<Item = Result<u8>>
{
    let error = Error::new(ErrorKind::Other, "Could not parse an event");
    match item {
        b'\x1B' => {
            // This is an escape character, leading a control sequence.
            Ok(match iter.next() {
                Some(Ok(b'O')) => {
                    match iter.next() {
                        // F1-F4
                        Some(Ok(val @ b'P'...b'S')) => Event::Key(Key::F(1 + val - b'P')),
                        _ => return Err(error),
                    }
                }
                Some(Ok(b'[')) => {
                    // This is a CSI sequence.
                    parse_csi(iter).ok_or(error)?
                }
                Some(Ok(c)) => {
                    let ch = try!(parse_utf8_char(c, iter));
                    Event::Key(Key::Alt(ch))
                }
                Some(Err(_)) | None => return Err(error),
            })
        }
        b'\n' | b'\r' => Ok(Event::Key(Key::Char('\n'))),
        b'\t' => Ok(Event::Key(Key::Char('\t'))),
        b'\x7F' => Ok(Event::Key(Key::Backspace)),
        c @ b'\x01'...b'\x1A' => Ok(Event::Key(Key::Ctrl((c as u8 - 0x1 + b'a') as char))),
        c @ b'\x1C'...b'\x1F' => Ok(Event::Key(Key::Ctrl((c as u8 - 0x1C + b'4') as char))),
        b'\0' => Ok(Event::Key(Key::Null)),
        c => {
            Ok({
                let ch = try!(parse_utf8_char(c, iter));
                Event::Key(Key::Char(ch))
            })
        }
    }
}

/// Parses a CSI sequence, just after reading ^[
///
/// Returns None if an unrecognized sequence is found.
fn parse_csi<I>(iter: &mut I) -> Option<Event>
    where I: Iterator<Item = Result<u8>>
{
    Some(match iter.next() {
        Some(Ok(b'D')) => Event::Key(Key::Left),
        Some(Ok(b'C')) => Event::Key(Key::Right),
        Some(Ok(b'A')) => Event::Key(Key::Up),
        Some(Ok(b'B')) => Event::Key(Key::Down),
        Some(Ok(b'H')) => Event::Key(Key::Home),
        Some(Ok(b'F')) => Event::Key(Key::End),
        Some(Ok(c @ b'0'...b'9')) => {
            // Numbered escape code.
            let mut buf = Vec::new();
            buf.push(c);
            let mut c = iter.next().unwrap().unwrap();
            // The final byte of a CSI sequence can be in the range 64-126, so
            // let's keep reading anything else.
            while c < 64 || c > 126 {
                buf.push(c);
                c = iter.next().unwrap().unwrap();
            }

            match c {
                // Special key code.
                b'~' => {
                    let str_buf = String::from_utf8(buf).unwrap();

                    // This CSI sequence can be a list of semicolon-separated
                    // numbers.
                    let nums: Vec<u8> = str_buf
                        .split(';')
                        .map(|n| n.parse().unwrap())
                        .collect();

                    if nums.is_empty() {
                        return None;
                    }

                    // TODO: handle multiple values for key modififiers (ex: values
                    // [3, 2] means Shift+Delete)
                    if nums.len() > 1 {
                        return None;
                    }

                    match nums[0] {
                        1 | 7 => Event::Key(Key::Home),
                        2 => Event::Key(Key::Insert),
                        3 => Event::Key(Key::Delete),
                        4 | 8 => Event::Key(Key::End),
                        5 => Event::Key(Key::PageUp),
                        6 => Event::Key(Key::PageDown),
                        v @ 11...15 => Event::Key(Key::F(v - 10)),
                        v @ 17...21 => Event::Key(Key::F(v - 11)),
                        v @ 23...24 => Event::Key(Key::F(v - 12)),
                        _ => return None,
                    }
                }
                _ => return None,
            }
        }
        _ => return None,
    })

}

/// Parse `c` as either a single byte ASCII char or a variable size UTF-8 char.
fn parse_utf8_char<I>(c: u8, iter: &mut I) -> Result<char>
    where I: Iterator<Item = Result<u8>>
{
    let error = Err(Error::new(ErrorKind::Other, "Input character is not valid UTF-8"));
    if c.is_ascii() {
        Ok(c as char)
    } else {
        let bytes = &mut Vec::new();
        bytes.push(c);

        loop {
            bytes.push(iter.next().unwrap().unwrap());
            if let Ok(st) = str::from_utf8(bytes) {
                return Ok(st.chars().next().unwrap());
            }
            if bytes.len() >= 4 {
                return error;
            }
        }
    }
}
