use std::io::{Read, Result};

use super::event::{self, Event, Key};

pub struct Events<R> {
    source: R,
    leftover: Option<u8>
}

impl<R: Read> Events<R> {
    pub fn new(source: R) -> Events<R> {
        Events {
            source: source,
            leftover: None
        }
    }
}

impl<R: Read> Iterator for Events<R> {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Result<Event>> {
        let mut source = &mut self.source;

        let mut buf = [0u8; 2];

        let result = match source.read(&mut buf) {
            Ok(0) => return None,
            Ok(1) => {
                match buf[0] {
                    b'\x1b' => Ok(Event::Key(Key::Escape)),
                    item => parse_event(item, &mut source.bytes())
                }
            },
            Ok(2) => {
                let mut option_iter = &mut Some(buf[1]).into_iter();

                let result = {
                    let mut iter = option_iter.
                        map(Ok).
                        chain(source.bytes());

                    parse_event(buf[0], &mut iter)
                };

                // If the option_iter wasn't consumed, keep the byte for later.
                self.leftover = option_iter.next();
                result
            },
            Ok(_) => unreachable!(),
            Err(e) => Err(e)
        };

        Some(result)
    }
}

fn parse_event<I>(item: u8, iter: &mut I) -> Result<Event>
    where I: Iterator<Item = Result<u8>>
{
    let mut buf = vec![item];
    let result = {
        let mut iter = iter.inspect(|byte| {
            if let Ok(byte) = *byte {
                buf.push(byte);
            }
        });
        event::parse_event(item, &mut iter)
    };
    result.or_else(|_| Ok(Event::Unknown(buf)))
}
