use std::fmt::Display;

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

#[derive(Debug, Clone, PartialEq)]
pub struct Q{
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

#[derive(Debug, Clone, PartialEq)]
pub struct T{
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