use std::fmt::Display;

use crate::parser::{point, trim};
use nom::branch::alt;

use nom::combinator::map;

use nom::{bytes::complete::tag, number::complete::float, sequence::pair};

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

impl LineTo {
    pub fn from_str(s: &str) -> nom::IResult<&str, LineTo> {
        alt((
            // L::from_str, H::from_str, V::from_str
            map(L::from_str, |v| v.into()),
            map(H::from_str, |v| v.into()),
            map(V::from_str, |v| v.into()),
        ))(s)
    }
}

impl From<L> for LineTo {
    fn from(l: L) -> Self {
        LineTo::L(l)
    }
}

impl From<H> for LineTo {
    fn from(h: H) -> Self {
        LineTo::H(h)
    }
}

impl From<V> for LineTo {
    fn from(v: V) -> Self {
        LineTo::V(v)
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

impl L {
    pub fn from_str(s: &str) -> nom::IResult<&str, L> {
        let (s, (relative, (x, y))) = trim(pair(trim(alt((tag("l"), tag("L")))), point))(s)?;

        Ok((
            s,
            L {
                x,
                y,
                relative: relative == "l",
            },
        ))
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

impl H {
    pub fn from_str(s: &str) -> nom::IResult<&str, H> {
        let (s, (relative, x)) = trim(pair(trim(alt((tag("h"), tag("H")))), float))(s)?;

        Ok((
            s,
            H {
                x,
                relative: relative == "h",
            },
        ))
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
mod test_line_to {
    use nom::multi::many0;

    use super::*;
    #[test]
    fn test_line_to() {
        let input = "L 90,90 V 10 H 50";
        let (s, l) = many0(LineTo::from_str)(input).unwrap();
        dbg!(s, l);
    }

    #[test]
    fn test_v() {
        assert_eq!(
            V::from_str("V 10"),
            Ok((
                "",
                V {
                    y: 10.0,
                    relative: false
                }
            ))
        );
        assert_eq!(
            V::from_str("v -10"),
            Ok((
                "",
                V {
                    y: -10.0,
                    relative: true
                }
            ))
        );
    }

    #[test]
    fn test_h() {
        assert_eq!(
            H::from_str("H 10"),
            Ok((
                "",
                H {
                    x: 10.0,
                    relative: false
                }
            ))
        );
        assert_eq!(
            H::from_str("h -10"),
            Ok((
                "",
                H {
                    x: -10.0,
                    relative: true
                }
            ))
        );
    }
    #[test]
    fn test_l() {
        assert_eq!(
            L::from_str("L10 20"),
            Ok((
                "",
                L {
                    x: 10.0,
                    y: 20.0,
                    relative: false
                }
            ))
        );
        assert_eq!(
            L::from_str("l -10,20"),
            Ok((
                "",
                L {
                    x: -10.0,
                    y: 20.0,
                    relative: true
                }
            ))
        );
    }
}
