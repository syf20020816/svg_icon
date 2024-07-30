use std::fmt::Display;

use nom::IResult;

use super::CommonKVs;

mod close_path;
mod command;
mod cubic_bezier;
mod elliptical_arc;
mod line_to;
mod move_to;
mod quadratic_bezier;

pub use close_path::Z;
pub use command::Command;
pub use cubic_bezier::*;
pub use elliptical_arc::A;
pub use line_to::*;
pub use move_to::M;
pub use quadratic_bezier::*;

#[derive(Debug, Clone, Default)]
pub struct Path {
    pub common: CommonKVs,
    pub d: Option<Command>,
}

impl Path {
    pub fn from_str(s: &str) -> IResult<&str, Path> {
        let (s, (common, kvs)) = CommonKVs::from_str(s.trim()).unwrap();

        if kvs.is_none() {
            Ok((s, Path { common, d: None }))
        } else {
            let d = kvs.as_ref().unwrap().get("d").map(|v| v.to_string());
            let d  = d.map(|v| Command::from_str(&v).unwrap().1);
            Ok((s, Path { common,d}))
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = if self.d.as_ref().is_some() {
            self.d.as_ref().unwrap().to_string()
        } else {
            "".to_string()
        };

        f.write_fmt(format_args!("<path d=\"{}\" {} />", d, self.common))
    }
}
