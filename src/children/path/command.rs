use std::fmt::Display;

use nom::IResult;

use super::{CubicBezier, LineTo, QuadraticBezier, A, M, Z};

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
    pub fn from_str(s: &str) -> IResult<&str, Command>{
        todo!();
        // let (s, c) = M::from_str(s).map(|(s, m)| (s, Command::MoveTo(m)))
        //     .or_else(|_| LineTo::from_str(s).map(|(s, l)| (s, Command::LineTo(l))))
        //     .or_else(|_| CubicBezier::from_str(s).map(|(s, c)| (s, Command::CubicBezier(c))))
        //     .or_else(|_| QuadraticBezier::from_str(s).map(|(s, q)| (s, Command::QuadraticBezier(q))))
        //     .or_else(|_| A::from_str(s).map(|(s, a)| (s, Command::EllipticalArc(a))))
        //     .or_else(|_| Z::from_str(s).map(|(s, z)| (s, Command::ClosePath(z)))?; // ? is used to return early if the parser fails
        // Ok((s, c))
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
