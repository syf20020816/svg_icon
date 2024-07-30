use std::fmt::Display;

use crate::parser::{point, trim};
use nom::branch::alt;
use nom::combinator::map;
use nom::{bytes::complete::tag, sequence::pair, IResult};

#[derive(Debug, Clone, PartialEq)]
pub enum CubicBezier {
    C(C),
    S(S),
}

impl Display for CubicBezier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CubicBezier::C(c) => write!(f, "{}", c),
            CubicBezier::S(s) => write!(f, "{}", s),
        }
    }
}

impl CubicBezier {
    pub fn from_str(s: &str) -> IResult<&str, CubicBezier> {
        alt((
            map(C::from_str, |v| v.into()),
            map(S::from_str, |v| v.into()),
        ))(s)
    }
}

impl From<C> for CubicBezier {
    fn from(c: C) -> Self {
        CubicBezier::C(c)
    }
}

impl From<S> for CubicBezier {
    fn from(s: S) -> Self {
        CubicBezier::S(s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct C {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub x: f32,
    pub y: f32,
    pub relative: bool,
}

impl Display for C {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {},{} {},{} {},{}",
            if self.relative { "c" } else { "C" },
            self.x1,
            self.y1,
            self.x2,
            self.y2,
            self.x,
            self.y
        )
    }
}

impl C {
    pub fn from_str(s: &str) -> IResult<&str, C> {
        let (s, (relative, ((x1, y1), ((x2, y2), (x, y))))) = trim(pair(
            trim(alt((tag("c"), tag("C")))),
            trim(pair(point, pair(point, point))),
        ))(s)?;

        Ok((
            s,
            C {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
                relative: relative == "c",
            },
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct S {
    pub x2: f32,
    pub y2: f32,
    pub x: f32,
    pub y: f32,
    pub relative: bool,
}

impl Display for S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {},{} {},{}",
            if self.relative { "s" } else { "S" },
            self.x2,
            self.y2,
            self.x,
            self.y
        )
    }
}

impl S {
    pub fn from_str(s: &str) -> IResult<&str, S> {
        let (s, (relative, ((x2, y2), (x, y)))) = trim(pair(
            trim(alt((tag("s"), tag("S")))),
            trim(pair(point, point)),
        ))(s)?;

        Ok((
            s,
            S {
                x2,
                y2,
                x,
                y,
                relative: relative == "s",
            },
        ))
    }
}

#[cfg(test)]
mod test_cubic {
    use nom::multi::many0;

    use super::*;

    #[test]
    fn test_cubic_bezier() {
        let (s, c) = many0(CubicBezier::from_str)("c 20,0 15,-80 40,-80 s 20,80 40,80").unwrap();
        dbg!(s, c);
    }

    #[test]
    fn test_c() {
        assert_eq!(
            C::from_str("c 10 10 20 20 30 30"),
            Ok((
                "",
                C {
                    x1: 10.0,
                    y1: 10.0,
                    x2: 20.0,
                    y2: 20.0,
                    x: 30.0,
                    y: 30.0,
                    relative: true
                }
            ))
        );

        assert_eq!(
            C::from_str("C 10,10 20,20 30,30"),
            Ok((
                "",
                C {
                    x1: 10.0,
                    y1: 10.0,
                    x2: 20.0,
                    y2: 20.0,
                    x: 30.0,
                    y: 30.0,
                    relative: false
                }
            ))
        );
    }

    #[test]
    fn test_s() {
        assert_eq!(
            S::from_str("s 10 10 20 20"),
            Ok((
                "",
                S {
                    x2: 10.0,
                    y2: 10.0,
                    x: 20.0,
                    y: 20.0,
                    relative: true
                }
            ))
        );

        assert_eq!(
            S::from_str("S 10,10 20,20"),
            Ok((
                "",
                S {
                    x2: 10.0,
                    y2: 10.0,
                    x: 20.0,
                    y: 20.0,
                    relative: false
                }
            ))
        );
    }
}
