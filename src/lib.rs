pub mod children;
pub mod encode;
pub mod header;
pub mod macros;
pub mod parser;
pub mod value;

use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use children::Child;
use header::Header;
use nom::bytes::complete::{tag, take_until1};

use nom::sequence::{delimited, preceded};
use parser::{parse_properties, trim};
use value::Auto;

/// # Svg
/// use `Svg::from_path` or `&str.parse().unwrap()` to parse a `Svg` from a file path.
/// ## Example
/// ```rust
/// let svg1 = Svg::from_path("E:/Rust/try/makepad/Gen-UI/gen/middleware/svg_icon/a.svg").unwrap();
/// let svg_str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100" fill="none"></svg>"#;
/// let svg2: Svg = svg_str.parse().unwrap();
/// let svg = svg!{
/// r##"<?xml version="1.0" encoding="UTF-8"?><svg width="24" height="24" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M24 19V4" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M12 22L24 19L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M28 30L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M44 30L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M20 36L12 22" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M4 36L12 22" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path fill-rule="evenodd" clip-rule="evenodd" d="M12 44C16.4183 44 20 40.4183 20 36H4C4 40.4183 7.58172 44 12 44Z" fill="#008000" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path fill-rule="evenodd" clip-rule="evenodd" d="M36 38C40.4183 38 44 34.4183 44 30H28C28 34.4183 31.5817 38 36 38Z" fill="#008000" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/></svg>"##
/// };
/// ```
#[derive(Debug, Default, Clone)]
pub struct Svg {
    pub header: Option<Header>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub height: Auto<f32>,
    pub width: Auto<f32>,
    pub view_box: Option<(u32, u32, u32, u32)>,
    pub fill: String,
    pub xmlns: String,
    pub children: Vec<Child>,
}

impl Svg {
    pub fn from_path<P>(path: P) -> Result<Self, String>
    where
        P: AsRef<std::path::Path>,
    {
        let s = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        dbg!(&s);
        s.parse()
    }
}

impl FromStr for Svg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut svg = Svg::default();
        // parse header -----------------------------------------------------------------------
        let (s, header) = if let Ok((s, header)) = Header::from_str(s) {
            (s, Some(header))
        } else {
            (s, None)
        };
        svg.header = header;
        // parse svg tag -----------------------------------------------------------------------
        let (s, props_str) = delimited(
            preceded(trim(tag("<")), tag("svg")),
            trim(take_until1(">")),
            trim(tag(">")),
        )(s)
        .unwrap();

        let kvs: Option<HashMap<&str, &str>> = if !props_str.is_empty() {
            let (remain, props) = parse_properties(props_str).unwrap();
            if !remain.is_empty() {
                return Err("Invalid svg tag props!".to_string());
            }
            Some(HashMap::from_iter(props.into_iter()))
        } else {
            None
        };

        if kvs.is_some() {
            svg.fill = kvs
                .as_ref()
                .unwrap()
                .get("fill")
                .unwrap_or(&"none")
                .to_string();
            svg.xmlns = kvs
                .as_ref()
                .unwrap()
                .get("xmlns")
                .unwrap_or(&"http://www.w3.org/2000/svg")
                .to_string();
            svg.x = kvs.as_ref().unwrap().get("x").map(|x| x.parse().unwrap());
            svg.y = kvs.as_ref().unwrap().get("y").map(|y| y.parse().unwrap());
            svg.height = Auto::get_from_map(&kvs.as_ref().unwrap(), "height");
            svg.width = Auto::get_from_map(&kvs.as_ref().unwrap(), "width");
            svg.view_box = kvs.as_ref().unwrap().get("viewBox").map(|view_box| {
                let mut iter = view_box.split_whitespace();
                let x = iter.next().unwrap().parse().unwrap();
                let y = iter.next().unwrap().parse().unwrap();
                let width = iter.next().unwrap().parse().unwrap();
                let height = iter.next().unwrap().parse().unwrap();
                (x, y, width, height)
            });
        }

        // children ----------------------------------------------------------------------------
        let (s, children) = Child::parser(s).unwrap();
        svg.children = children;
        let (s, _) = delimited(
            preceded(trim(tag("<")), tag("/")),
            trim(tag("svg")),
            trim(tag(">")),
        )(s)
        .unwrap();
        if !s.is_empty() {
            return Err(format!("Invalid svg tag: {}", s));
        }
        Ok(svg)
    }
}

impl Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if let Some(header) = &self.header {
            s.push_str(&header.to_string());
        }
        s.push_str("<svg ");
        if !self.fill.is_empty() {
            s.push_str(&format!("fill=\"{}\" ", self.fill));
        }
        if !self.xmlns.is_empty() {
            s.push_str(&format!("xmlns=\"{}\" ", self.xmlns));
        }
        if let Some(x) = self.x {
            s.push_str(&format!("x=\"{}\" ", x));
        }
        if let Some(y) = self.y {
            s.push_str(&format!("y=\"{}\" ", y));
        }
        if let Some(view_box) = self.view_box {
            s.push_str(&format!(
                "viewBox=\"{} {} {} {}\" ",
                view_box.0, view_box.1, view_box.2, view_box.3
            ));
        }
        s.push_str(&format!("height=\"{}\" ", self.height));
        s.push_str(&format!("width=\"{}\" ", self.width));

        s.push_str(">");
        for child in &self.children {
            s.push_str(&child.to_string());
        }
        s.push_str("</svg>");
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test_svg {

    #[test]
    fn test_svg() {
        use super::Svg;
        use crate::svg;

        let svg = svg! {
            r##"<?xml version="1.0" encoding="UTF-8"?><svg width="24" height="24" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M24 19V4" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M12 22L24 19L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M28 30L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M44 30L36 16" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M20 36L12 22" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path d="M4 36L12 22" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path fill-rule="evenodd" clip-rule="evenodd" d="M12 44C16.4183 44 20 40.4183 20 36H4C4 40.4183 7.58172 44 12 44Z" fill="#008000" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/><path fill-rule="evenodd" clip-rule="evenodd" d="M36 38C40.4183 38 44 34.4183 44 30H28C28 34.4183 31.5817 38 36 38Z" fill="#008000" stroke="#008000" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/></svg>"##
        };
        dbg!(svg);
        let svg =
            Svg::from_path("E:/Rust/try/makepad/Gen-UI/gen/middleware/svg_icon/a.svg").unwrap();
        dbg!(svg.to_string());
        dbg!(svg);
        let svg_str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100" fill="none"></svg>"#;
        let svg2: Svg = svg_str.parse().unwrap();
        dbg!(svg2);
    }
}
