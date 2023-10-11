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

    pub fn to_str(&self) -> String {
        let name_str = NoteName::to_str(&self.name);
        let accidental_str = Accidental::to_str(&self.accidental);
        let octave_str = format!("{}", &self.octave);

        let mut note_str = String::new();
        note_str.push_str(name_str);
        note_str.push_str(accidental_str);
        note_str.push_str(&octave_str.as_str());
        note_str
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

    #[test]
    fn to_str() {
        use NoteName::*;
        use Accidental::*;

        let note = Note {name: C, accidental: Flat, octave: 0};
        assert_eq!(note.to_str(), "Cb0");
        let note = Note {name: D, accidental: Natural, octave: 1};
        assert_eq!(note.to_str(), "D1");
        let note = Note {name: E, accidental: Sharp, octave: 2};
        assert_eq!(note.to_str(), "E#2");
        let note = Note {name: F, accidental: Flat, octave: 3};
        assert_eq!(note.to_str(), "Fb3");
        let note = Note {name: G, accidental: Natural, octave: 4};
        assert_eq!(note.to_str(), "G4");
        let note = Note {name: A, accidental: Sharp, octave: 5};
        assert_eq!(note.to_str(), "A#5");
        let note = Note {name: B, accidental: Flat, octave: 6};
        assert_eq!(note.to_str(), "Bb6");
    }
}
