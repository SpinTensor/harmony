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

        // notename
        let name_start_idx = 0;
        let name_end_idx = name_start_idx+1;
        if s.len() >= name_end_idx {
            note.name = NoteName::from_str(&s[name_start_idx..name_end_idx])?;
        } else {
            return Err("Unable to parse note");
        }
        
        // accidental
        let accicental_start_idx = name_end_idx;
        let mut accicental_end_idx = accicental_start_idx;
        for c in s[accicental_end_idx..].chars() {
            if c.is_numeric() {
                break;
            } else {
                accicental_end_idx += 1;
            }
        }
        if s.len() >= accicental_end_idx {
            note.accidental = Accidental::from_str(&s[accicental_start_idx..accicental_end_idx])?
        } else {
            return Err("Unable to parse accidental");
        }

        // octave
        let octave_start_idx = accicental_end_idx;
        let octave_end_idx = s.len();
        match s[octave_start_idx..octave_end_idx].parse::<u8>() {
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

    //pub fn next(&mut self) -> Self {
    //    let mut next_note = Note::default();
    //    
    //}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {
        use NoteName::*;
        use Accidental::*;

        assert_eq!(Note::from_str("Abb1"), Ok(Note {name: A, accidental: Doubleflat, octave: 1}));
        assert_eq!(Note::from_str("Db4"), Ok(Note {name: D, accidental: Flat, octave: 4}));
        assert_eq!(Note::from_str("E2"), Ok(Note {name: E, accidental: Natural, octave: 2}));
        assert_eq!(Note::from_str("F#5"), Ok(Note {name: F, accidental: Sharp, octave: 5}));
        assert_eq!(Note::from_str("G##6"), Ok(Note {name: G, accidental: Doublesharp, octave: 6}));

        assert!(Note::from_str("C#").is_err());
        assert!(Note::from_str("Bbb").is_err());
        assert!(Note::from_str("Hb3").is_err());
        assert!(Note::from_str("B!3").is_err());
        assert!(Note::from_str("G3b").is_err());
        assert!(Note::from_str("#3").is_err());
        assert!(Note::from_str("3").is_err());
        assert!(Note::from_str("").is_err());
    }

    #[test]
    fn to_str() {
        use NoteName::*;
        use Accidental::*;

        let note = Note {name: C, accidental: Doubleflat, octave: 0};
        assert_eq!(note.to_str(), "Cbb0");
        let note = Note {name: D, accidental: Flat, octave: 1};
        assert_eq!(note.to_str(), "Db1");
        let note = Note {name: E, accidental: Natural, octave: 2};
        assert_eq!(note.to_str(), "E2");
        let note = Note {name: F, accidental: Sharp, octave: 3};
        assert_eq!(note.to_str(), "F#3");
        let note = Note {name: G, accidental: Doublesharp, octave: 4};
        assert_eq!(note.to_str(), "G##4");
        let note = Note {name: A, accidental: Natural, octave: 5};
        assert_eq!(note.to_str(), "A5");
        let note = Note {name: B, accidental: Natural, octave: 6};
        assert_eq!(note.to_str(), "B6");
    }
}
