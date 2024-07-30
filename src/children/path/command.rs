use std::fmt::Display;

use super::{CubicBezier, LineTo, QuadraticBezier, A, M, Z};
use nom::branch::alt;
use nom::{combinator::map, IResult};

/// Required. A set of commands which define the path.
///
/// The following commands are available for path data:
///
/// - M = moveto (move from one point to another point)
/// - L = lineto (create a line)
/// - H = horizontal lineto (create a horizontal line)
/// - V = vertical lineto (create a vertical line)
/// - C = curveto (create a curve)
/// - S = smooth curveto (create a smooth curve)
/// - Q = quadratic Bézier curve (create a quadratic Bézier curve)
/// - T = smooth quadratic Bézier curveto (create a smooth quadratic Bézier curve)
/// - A = elliptical Arc (create a elliptical arc)
/// - Z = closepath (close the path)
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    MoveTo(M),
    LineTo(LineTo),
    CubicBezier(CubicBezier),
    QuadraticBezier(QuadraticBezier),
    EllipticalArc(A),
    ClosePath(Z),
}

impl Command {
    pub fn from_str(s: &str) -> IResult<&str, Command> {
        alt((
            map(M::from_str, |v| v.into()),
            map(LineTo::from_str, |v| v.into()),
            map(CubicBezier::from_str, |v| v.into()),
            map(QuadraticBezier::from_str, |v| v.into()),
            map(A::from_str, |v| v.into()),
            map(Z::from_str, |v| v.into()),
        ))(s)
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::MoveTo(m) => write!(f, "{}", m),
            Command::LineTo(l) => write!(f, "{}", l),
            Command::CubicBezier(c) => write!(f, "{}", c),
            Command::QuadraticBezier(q) => write!(f, "{}", q),
            Command::EllipticalArc(a) => write!(f, "{}", a),
            Command::ClosePath(z) => write!(f, "{}", z),
        }
    }
}

impl From<M> for Command {
    fn from(m: M) -> Self {
        Command::MoveTo(m)
    }
}

impl From<LineTo> for Command {
    fn from(l: LineTo) -> Self {
        Command::LineTo(l)
    }
}

impl From<CubicBezier> for Command {
    fn from(c: CubicBezier) -> Self {
        Command::CubicBezier(c)
    }
}

impl From<QuadraticBezier> for Command {
    fn from(q: QuadraticBezier) -> Self {
        Command::QuadraticBezier(q)
    }
}

impl From<A> for Command {
    fn from(a: A) -> Self {
        Command::EllipticalArc(a)
    }
}

impl From<Z> for Command {
    fn from(z: Z) -> Self {
        Command::ClosePath(z)
    }
}


#[cfg(test)]
mod test_command{
    use nom::multi::many0;

    use super::*;

    #[test]
    fn test1(){
        assert_eq!(
            Command::from_str("M 10 10"),
            Ok((
                "",
                Command::MoveTo(M {
                    x: 10.0,
                    y: 10.0,
                    relative: false,
                })
            ))
        );
    }

    #[test]
    fn test_full(){
        let input = "M 10 10 L 20 20 C 30 30 40 40 50 50 Q 60 60 70 70 A 80 80 90 0 1 100 100 Z";
        let (s, c) = many0(Command::from_str)(input).unwrap();
        dbg!(s, c);
    }
}