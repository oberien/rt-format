#[macro_use]
mod codegen;

pub mod map;
pub mod parser;

use paste::paste;
use regex::{Captures, Match};
use std::cmp::PartialEq;
use std::fmt;

use crate::map::Map;

generate_code! {
    align: Align {
        "" => None,
        "<" => Left,
        "^" => Center,
        ">" => Right,
    }

    sign: Sign {
        "" => Default,
        "+" => Always,
    }

    repr: Repr {
        "" => Default,
        "#" => Alt,
    }

    pad: Pad {
        "" => Space,
        "0" => Zero,
    }

    width: Width {
        "" => Auto,
        "width$" => AtLeast { width: usize },
    }

    precision: Precision {
        "" => Auto,
        ".precision$" => Exactly { precision: usize },
    }

    format: Format {
        "" => Display,
        "?" => Debug,
        "o" => Octal,
        "x" => LowerHex,
        "X" => UpperHex,
        "b" => Binary,
        "e" => LowerExp,
        "E" => UpperExp,
    }
}

impl Width {
    fn from_capture(capture: Option<Match>) -> Result<Self, ()> {
        match capture.map(|m| m.as_str()).unwrap_or("") {
            "" => Ok(Width::Auto),
            s@_ => match s.parse() {
                Ok(width) => Ok(Width::AtLeast { width }),
                Err(_) => Err(())
            }
        }
    }
}

impl Precision {
    fn from_capture(capture: Option<Match>) -> Result<Self, ()> {
        match capture.map(|m| m.as_str()).unwrap_or("") {
            "" => Ok(Precision::Auto),
            s@_ => match s.parse() {
                Ok(precision) => Ok(Precision::Exactly { precision }),
                Err(_) => Err(())
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Segment<'s, V: FormattableValue> {
    Text(&'s str),
    Argument(Argument<'s, V>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arguments<'a, V: FormattableValue> {
    pub segments: Vec<Segment<'a, V>>
}

impl<'a, V: FormattableValue> Arguments<'a, V> {
    pub fn parse<M>(format: &'a str, positional: &'a [V], named: &'a M) -> Result<Self, usize>
    where
        M: Map<str, V>
    {
        use parser::Parser;

        let segments: Result<Vec<Segment<'a, V>>, usize> = Parser::new(format, positional, named).collect();
        Ok(Arguments { segments: segments? })
    }
}

impl<'a, V: FormattableValue> fmt::Display for Arguments<'a, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for segment in self.segments.iter() {
            match segment {
                Segment::Text(text) => f.write_str(text)?,
                Segment::Argument(arg) => arg.fmt(f)?
            }
        }
        Ok(())
    }
}
