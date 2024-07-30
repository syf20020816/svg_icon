use std::fmt::Display;

use crate::parser::{point, trim};
use nom::branch::alt;
use nom::combinator::map;
use nom::{bytes::complete::tag, sequence::pair};

#[derive(Debug, Clone, PartialEq)]
pub enum QuadraticBezier {
    Q(Q),
    T(T),
}

impl Display for QuadraticBezier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuadraticBezier::Q(q) => write!(f, "{}", q),
            QuadraticBezier::T(t) => write!(f, "{}", t),
        }
    }
}

impl QuadraticBezier {
    pub fn from_str(s: &str) -> nom::IResult<&str, QuadraticBezier> {
        alt((
            map(Q::from_str, |v| v.into()),
            map(T::from_str, |v| v.into()),
        ))(s)
    }
    
}

impl From<Q> for QuadraticBezier {
    fn from(q: Q) -> Self {
        QuadraticBezier::Q(q)
    }
}

impl From<T> for QuadraticBezier {
    fn from(t: T) -> Self {
        QuadraticBezier::T(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Q {
    pub x1: f32,
    pub y1: f32,
    pub x: f32,
    pub y: f32,
    pub relative: bool,
}

impl Display for Q {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {},{} {},{}",
            if self.relative { "q" } else { "Q" },
            self.x1,
            self.y1,
            self.x,
            self.y
        )
    }
}

impl Q {
    pub fn from_str(s: &str) -> nom::IResult<&str, Q> {
        let (s, (relative, ((x1, y1), (x, y)))) = trim(pair(
            trim(alt((tag("q"), tag("Q")))),
            trim(pair(point, point)),
        ))(s)?;

        Ok((
            s,
            Q {
                x1,
                y1,
                x,
                y,
                relative: relative == "q",
            },
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct T {
    pub x: f32,
    pub y: f32,
    pub relative: bool,
}

impl Display for T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {},{}",
            if self.relative { "t" } else { "T" },
            self.x,
            self.y
        )
    }
}

impl T {
    pub fn from_str(s: &str) -> nom::IResult<&str, T> {
        let (s, (relative, (x, y))) = trim(pair(trim(alt((tag("t"), tag("T")))), point))(s)?;

        Ok((
            s,
            T {
                x,
                y,
                relative: relative == "t",
            },
        ))
    }
}

#[cfg(test)]
mod test_quadratic_bezier {
    use super::*;
    #[test]
    fn test_q() {
        assert_eq!(
            Q::from_str("Q 5 5 10 10"),
            Ok((
                "",
                Q {
                    x1: 5.0,
                    y1: 5.0,
                    x: 10.0,
                    y: 10.0,
                    relative: false,
                }
            ))
        );
        assert_eq!(
            Q::from_str("q 25,25 40,50"),
            Ok((
                "",
                Q {
                    x1: 25.0,
                    y1: 25.0,
                    x: 40.0,
                    y: 50.0,
                    relative: true,
                }
            ))
        );
    }

    #[test]
    fn test_t() {
        assert_eq!(
            T::from_str("t 10 10"),
            Ok((
                "",
                T {
                    x: 10.0,
                    y: 10.0,
                    relative: true,
                }
            ))
        )
    }
}
