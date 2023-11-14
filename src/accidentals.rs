#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Accidental {
    Doubleflat,
    Flat,
    Natural,
    Sharp,
    Doublesharp
}

impl Accidental {
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "bb" => Ok(Accidental::Doubleflat),
            "b" => Ok(Accidental::Flat),
            "" => Ok(Accidental::Natural),
            "#" => Ok(Accidental::Sharp),
            "##" => Ok(Accidental::Doublesharp),
            _ => Err("Invalid Accidental symbol"),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Accidental::Doubleflat => "bb",
            Accidental::Flat => "b",
            Accidental::Natural => "",
            Accidental::Sharp => "#",
            Accidental::Doublesharp => "##",
        }
    }

    pub fn offset(&self) -> i32 {
        match self {
            Accidental::Doubleflat => -2,
            Accidental::Flat => -1,
            Accidental::Natural => 0,
            Accidental::Sharp => 1,
            Accidental::Doublesharp => 2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(Accidental::from_str("bb"), Ok(Accidental::Doubleflat));
        assert_eq!(Accidental::from_str("b"), Ok(Accidental::Flat));
        assert_eq!(Accidental::from_str(""), Ok(Accidental::Natural));
        assert_eq!(Accidental::from_str("#"), Ok(Accidental::Sharp));
        assert_eq!(Accidental::from_str("##"), Ok(Accidental::Doublesharp));

        assert!(Accidental::from_str("B").is_err());
        assert!(Accidental::from_str("bB").is_err());
        assert!(Accidental::from_str("§").is_err());
        assert!(Accidental::from_str("This should fail").is_err());
    }

    #[test]
    fn to_str() {
        assert_eq!(Accidental::to_str(&Accidental::Doubleflat), "bb");
        assert_eq!(Accidental::to_str(&Accidental::Flat), "b");
        assert_eq!(Accidental::to_str(&Accidental::Natural), "");
        assert_eq!(Accidental::to_str(&Accidental::Sharp), "#");
        assert_eq!(Accidental::to_str(&Accidental::Doublesharp), "##");
    }

    #[test]
    fn offset() {
        assert_eq!(Accidental::Doubleflat.offset(), -2);
        assert_eq!(Accidental::Flat.offset(), -1);
        assert_eq!(Accidental::Natural.offset(), 0);
        assert_eq!(Accidental::Sharp.offset(), 1);
        assert_eq!(Accidental::Doublesharp.offset(), 2);
    }
}
