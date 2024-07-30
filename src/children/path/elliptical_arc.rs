use std::fmt::Display;

use crate::parser::{bool_flag, point, trim};
use nom::branch::alt;
use nom::combinator::map;
use nom::{bytes::complete::tag, number::complete::float, sequence::pair};

/// Draw an Arc curve from the current point to the coordinate x,y.
#[derive(Debug, Clone, PartialEq)]
pub struct A {
    /// rx and ry are the two radii of the ellipse;
    pub rx: f32,
    /// rx and ry are the two radii of the ellipse;
    pub ry: f32,
    /// represents a rotation (in degrees) of the ellipse relative to the x-axis;
    pub angle: f32,
    /// allows to chose one of the large arc (1) or small arc (0),
    pub large_arc_flag: bool,
    /// allows to chose one of the clockwise turning arc (1) or counterclockwise turning arc (0)
    pub sweep_flag: bool,
    /// x and y are the coordinates of the end point of the arc;
    pub x: f32,
    pub y: f32,
    pub relative: bool,
}

impl Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {},{}",
            if self.relative { "a" } else { "A" },
            self.rx,
            self.ry,
            self.angle,
            if self.large_arc_flag { "1" } else { "0" },
            if self.sweep_flag { "1" } else { "0" },
            self.x,
            self.y
        )
    }
}

impl A {
    pub fn from_str(s: &str) -> nom::IResult<&str, A> {
        let (s, (relative, ((rx, ry), (angle, (large_arc_flag, (sweep_flag, (x, y))))))) =
            trim(pair(
                trim(alt((tag("a"), tag("A")))),
                trim(pair(
                    point,
                    pair(float, pair(bool_flag, pair(bool_flag, point))),
                )),
            ))(s)?;

        Ok((
            s,
            A {
                rx,
                ry,
                angle,
                large_arc_flag: large_arc_flag,
                sweep_flag: sweep_flag,
                x,
                y,
                relative: relative == "a",
            },
        ))
    }
}

#[cfg(test)]
mod test_elliptical_arc {
    use super::*;
    #[test]
    fn test_a() {
        assert_eq!(
            A::from_str("A 5 5 0 0 1 10 10"),
            Ok((
                "",
                A {
                    rx: 5.0,
                    ry: 5.0,
                    angle: 0.0,
                    large_arc_flag: false,
                    sweep_flag: true,
                    x: 10.0,
                    y: 10.0,
                    relative: false,
                }
            ))
        );

        assert_eq!(
            A::from_str("A 6 4 10 0 1 14,10"),
            Ok((
                "",
                A {
                    rx: 6.0,
                    ry: 4.0,
                    angle: 10.0,
                    large_arc_flag: false,
                    sweep_flag: true,
                    x: 14.0,
                    y: 10.0,
                    relative: false,
                }
            ))
        );
    }
}
