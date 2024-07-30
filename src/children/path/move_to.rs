use std::fmt::Display;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::pair;
use nom::{character::complete::space1, number::complete::float, sequence::separated_pair};

use crate::parser::trim;
/// MoveTo instructions can be thought of as picking up the drawing instrument,
/// and setting it down somewhere else—in other words, moving the current point (Po; {xo, yo}).
/// There is no line drawn between Po and the new current point (Pn; {xn, yn}).
#[derive(Debug, Clone, PartialEq)]
pub struct M {
    pub x: f32,
    pub y: f32,
    /// If the command is relative, all the coordinates are relative to the current point.
    pub relative: bool,
}

impl M {
    pub fn from_str(s: &str) -> nom::IResult<&str, M> {
        let (s, (relative, (x, y))) = trim(pair(
            trim(alt((tag("m"), tag("M")))),
            trim(separated_pair(float, alt((tag(","), space1)), float)),
        ))(s)?;

        Ok((
            s,
            M {
                x,
                y,
                relative: relative == "m",
            },
        ))
    }
}

impl Display for M {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {},{}",
            if self.relative { "m" } else { "M" },
            self.x,
            self.y
        )
    }
}

#[cfg(test)]
mod test_move_to {
    use super::*;

    #[test]
    fn test_move_to() {
        assert_eq!(
            M::from_str("M 10,20 l 80, 80"),
            Ok((
                "l 80, 80",
                M {
                    x: 10.0,
                    y: 20.0,
                    relative: false
                }
            ))
        );
        assert_eq!(
            M::from_str("m 10,20"),
            Ok((
                "",
                M {
                    x: 10.0,
                    y: 20.0,
                    relative: true
                }
            ))
        );
        dbg!(M::from_str("M 10 20").unwrap().1.to_string());
    }
}
