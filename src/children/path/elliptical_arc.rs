use std::fmt::Display;

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
