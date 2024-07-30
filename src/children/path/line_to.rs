use std::fmt::Display;

use crate::parser::trim;
use nom::branch::alt;
use nom::{
    bytes::complete::tag, character::complete::one_of, combinator::recognize,
    number::complete::float, sequence::pair,
};

#[derive(Debug, Clone, PartialEq)]
pub enum LineTo {
    L(L),
    H(H),
    V(V),
}

impl Display for LineTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineTo::L(l) => write!(f, "{}", l),
            LineTo::H(h) => write!(f, "{}", h),
            LineTo::V(v) => write!(f, "{}", v),
        }
    }
}

/// L = lineto (create a line)
#[derive(Debug, Clone, PartialEq)]
pub struct L {
    pub x: f32,
    pub y: f32,
    pub relative: bool,
}

impl Display for L {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {},{}",
            if self.relative { "l" } else { "L" },
            self.x,
            self.y
        )
    }
}

/// H = horizontal lineto (create a horizontal line)
#[derive(Debug, Clone, PartialEq)]
pub struct H {
    pub x: f32,
    pub relative: bool,
}

impl Display for H {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", if self.relative { "h" } else { "H" }, self.x)
    }
}

/// V = vertical lineto (create a vertical line)
#[derive(Debug, Clone, PartialEq)]
pub struct V {
    pub y: f32,
    pub relative: bool,
}

impl Display for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", if self.relative { "v" } else { "V" }, self.y)
    }
}

impl V {
    pub fn from_str(s: &str) -> nom::IResult<&str, V> {
        let (s, (relative, y)) = trim(pair(trim(alt((tag("v"), tag("V")))), float))(s)?;

        Ok((
            s,
            V {
                y,
                relative: relative == "v",
            },
        ))
    }
}

#[cfg(test)]
mod test_line_to{
    use super::*;
    #[test]
    fn test_line_to() {
        assert_eq!(
            LineTo::L(L { x: 10.0, y: 20.0, relative: false }).to_string(),
            "L 10,20"
        );
        assert_eq!(
            LineTo::L(L { x: 10.0, y: 20.0, relative: true }).to_string(),
            "l 10,20"
        );
        assert_eq!(
            LineTo::H(H { x: 10.0, relative: false }).to_string(),
            "H 10"
        );
        assert_eq!(
            LineTo::H(H { x: 10.0, relative: true }).to_string(),
            "h 10"
        );
        assert_eq!(
            LineTo::V(V { y: 10.0, relative: false }).to_string(),
            "V 10"
        );
        assert_eq!(
            LineTo::V(V { y: 10.0, relative: true }).to_string(),
            "v 10"
        );
    }
}