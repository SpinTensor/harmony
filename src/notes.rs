use crate::notenames::NoteName;
use crate::accidentals::Accidental;

#[derive(Debug, PartialEq)]
pub struct Note {
    name: NoteName,
    accidental: Accidental,
    octave: u8,
}

impl Note {
    pub fn default() -> Self {
        Self {
            name: NoteName::C,
            accidental: Accidental::Natural,
            octave: 3,
        }
    }

    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        let mut note = Self::default();
        if s.len() > 0 {
            note.name = NoteName::from_str(&s[0..1])?;
        } else {
            return Err("Unable to parse note");
        }

        let has_accidental = s.len() > 1 &&
            s[1..].chars().map(|c| c.is_numeric()).collect::<Vec<bool>>().contains(&false);
        if has_accidental {
            note.accidental = Accidental::from_str(&s[1..2])?;
        }
        let octave_str = if has_accidental {&s[2..]} else {&s[1..]};
        match octave_str.parse::<u8>() {
            Ok(oct) => note.octave = oct,
            Err(_) => return Err("Invalid Octave")
        }
        Ok(note)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {
        use NoteName::*;
        use Accidental::*;

        assert_eq!(Note::from_str("Db4"), Ok(Note {name: D, accidental: Flat, octave: 4}));
        assert_eq!(Note::from_str("E#2"), Ok(Note {name: E, accidental: Sharp, octave: 2}));
        assert_eq!(Note::from_str("F5"), Ok(Note {name: F, accidental: Natural, octave: 5}));

        assert!(Note::from_str("Bbb").is_err());
        assert!(Note::from_str("Hb3").is_err());
        assert!(Note::from_str("B!3").is_err());
        assert!(Note::from_str("#3").is_err());
        assert!(Note::from_str("3").is_err());
        assert!(Note::from_str("").is_err());
    }
}
