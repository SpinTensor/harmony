#[derive(Debug, PartialEq)]
pub enum Accidental {
    Flat,
    Natural,
    Sharp
}

impl Accidental {
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "b" => Ok(Accidental::Flat),
            "" => Ok(Accidental::Natural),
            "#" => Ok(Accidental::Sharp),
            _ => Err("Invalid Accidental symbol"),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Accidental::Flat => "b",
            Accidental::Natural => "",
            Accidental::Sharp => "#",
        }
    }

    pub fn offset(&self) -> i32 {
        match self {
            Accidental::Flat => -1,
            Accidental::Natural => 0,
            Accidental::Sharp => 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(Accidental::from_str("b"), Ok(Accidental::Flat));
        assert_eq!(Accidental::from_str(""), Ok(Accidental::Natural));
        assert_eq!(Accidental::from_str("#"), Ok(Accidental::Sharp));

        assert!(Accidental::from_str("B").is_err());
        assert!(Accidental::from_str("§").is_err());
        assert!(Accidental::from_str("This should fail").is_err());
    }

    #[test]
    fn to_str() {
        assert_eq!(Accidental::to_str(&Accidental::Flat), "b");
        assert_eq!(Accidental::to_str(&Accidental::Natural), "");
        assert_eq!(Accidental::to_str(&Accidental::Sharp), "#");
    }

    #[test]
    fn offset() {
        assert_eq!(Accidental::Flat.offset(), -1);
        assert_eq!(Accidental::Natural.offset(), 0);
        assert_eq!(Accidental::Sharp.offset(), 1);
    }
}
