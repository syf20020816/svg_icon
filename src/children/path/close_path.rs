use std::fmt::Display;

use nom::{branch::alt, character::complete::char};

#[derive(Debug, Clone, PartialEq)]
pub struct Z(bool);

impl Z {
    pub fn from_str(s: &str) -> nom::IResult<&str, Z> {
        let (s, releative) = alt((char('z'), char('Z')))(s)?;
        Ok((s, Z(releative == 'z')))
    }
}

impl Display for Z {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Z")
    }
}

#[cfg(test)]
mod test_close{
    use super::*;
    #[test]
    fn test_close() {
        assert_eq!(
            Z::from_str("Z"),
            Ok((
                "",
                Z {
                    0: false
                }
            ))
        );
        assert_eq!(
            Z::from_str("z"),
            Ok((
                "",
                Z {
                    0: true
                }
            ))
        );
    }
}