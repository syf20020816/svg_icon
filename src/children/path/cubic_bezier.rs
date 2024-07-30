use std::fmt::Display;

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

#[derive(Debug, Clone, PartialEq)]
pub struct C{
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

#[derive(Debug, Clone, PartialEq)]
pub struct S{
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