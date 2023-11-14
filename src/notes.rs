use crate::notenames::NoteName;
use crate::accidentals::Accidental;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Note {
    name: NoteName,
    accidental: Accidental,
    octave: i8,
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
            if c.is_numeric() || c == '-' {
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
        match s[octave_start_idx..octave_end_idx].parse::<i8>() {
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

    pub fn dist_hsteps(&self, other: &Note) -> i32 {
        12*(other.octave as i32 - self.octave as i32)
            + self.name.dist_hsteps(&other.name)
            + other.accidental.offset() - self.accidental.offset()
    }

    pub fn rm_accidental(&self) -> Self {
        self.set_accidental(Accidental::Natural)
    }

    pub fn set_accidental(&self, accidental: Accidental) -> Self {
        let mut accidented_note = *self;
        accidented_note.accidental = accidental;
        accidented_note
    }

    pub fn next_natural(&self) -> Self {
        let mut next_note = *self;
        match next_note.name {
            NoteName::B => next_note.octave += 1,
            _ => ()
        }
        next_note.name = next_note.name.next();
        next_note = next_note.rm_accidental();
        next_note
    }

    pub fn prev_natural(&self) -> Self {
        let mut prev_note = *self;
        match prev_note.name {
            NoteName::C => prev_note.octave -= 1,
            _ => ()
        }
        prev_note.name = prev_note.name.prev();
        prev_note = prev_note.rm_accidental();
        prev_note
    }
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
        assert_eq!(Note::from_str("C-2"), Ok(Note {name: C, accidental: Natural, octave: -2}));
        assert_eq!(Note::from_str("d#-1"), Ok(Note {name: D, accidental: Sharp, octave: -1}));
        assert_eq!(Note::from_str("Bbb-3"), Ok(Note {name: B, accidental: Doubleflat, octave: -3}));

        assert!(Note::from_str("C#").is_err());
        assert!(Note::from_str("Bbb").is_err());
        assert!(Note::from_str("Hb3").is_err());
        assert!(Note::from_str("B!3").is_err());
        assert!(Note::from_str("G3b").is_err());
        assert!(Note::from_str("C-#2").is_err());
        assert!(Note::from_str("E-").is_err());
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

    #[test]
    fn dist_hsteps() {
        // Octave differs
        let note1 = Note::from_str("C3").unwrap();
        let note2 = Note::from_str("C4").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), 12);
        let note1 = Note::from_str("C4").unwrap();
        let note2 = Note::from_str("C3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), -12);

        // Notevalue differs
        let note1 = Note::from_str("C3").unwrap();
        let note2 = Note::from_str("D3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), 2);
        let note1 = Note::from_str("D3").unwrap();
        let note2 = Note::from_str("C3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), -2);

        // Notevalue and Octave differs
        let note1 = Note::from_str("C3").unwrap();
        let note2 = Note::from_str("D4").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), 14);
        let note1 = Note::from_str("C4").unwrap();
        let note2 = Note::from_str("D3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), -10);
        let note1 = Note::from_str("F3").unwrap();
        let note2 = Note::from_str("D4").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), 9);

        // Accidentals differ
        let note1 = Note::from_str("C3").unwrap();
        let note2 = Note::from_str("C#3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), 1);
        let note1 = Note::from_str("C3").unwrap();
        let note2 = Note::from_str("Cb3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), -1);
        let note1 = Note::from_str("C#3").unwrap();
        let note2 = Note::from_str("C3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), -1);
        let note1 = Note::from_str("Cb3").unwrap();
        let note2 = Note::from_str("C3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), 1);
        let note1 = Note::from_str("Cb3").unwrap();
        let note2 = Note::from_str("Cbb3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), -1);
        let note1 = Note::from_str("Cbb3").unwrap();
        let note2 = Note::from_str("Cb3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), 1);
        let note1 = Note::from_str("C#3").unwrap();
        let note2 = Note::from_str("C##3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), 1);
        let note1 = Note::from_str("C##3").unwrap();
        let note2 = Note::from_str("C#3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), -1);
        let note1 = Note::from_str("Cb3").unwrap();
        let note2 = Note::from_str("C#3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), 2);
        let note1 = Note::from_str("C#3").unwrap();
        let note2 = Note::from_str("Cb3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), -2);

        // Notevalue and Accidental differ
        let note1 = Note::from_str("G#3").unwrap();
        let note2 = Note::from_str("A3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), 1);
        let note1 = Note::from_str("A3").unwrap();
        let note2 = Note::from_str("G#3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), -1);

        // All differ
        let note1 = Note::from_str("Fbb4").unwrap();
        let note2 = Note::from_str("D#3").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), -12);
        let note1 = Note::from_str("F#3").unwrap();
        let note2 = Note::from_str("D##4").unwrap();
        assert_eq!(note1.dist_hsteps(&note2), 10);
    }

    #[test]
    fn rm_accidental() {
        let doubleflat_note = Note::from_str("Cbb3").unwrap();
        let flat_note = Note::from_str("Cb3").unwrap();
        let natural_note = Note::from_str("C3").unwrap();
        let sharp_note = Note::from_str("C#3").unwrap();
        let doublesharp_note = Note::from_str("C##3").unwrap();

        assert_eq!(doubleflat_note.rm_accidental(), natural_note);
        assert_eq!(flat_note.rm_accidental(), natural_note);
        assert_eq!(natural_note.rm_accidental(), natural_note);
        assert_eq!(sharp_note.rm_accidental(), natural_note);
        assert_eq!(doublesharp_note.rm_accidental(), natural_note);
    }

    #[test]
    fn set_accidental() {
        let doubleflat_note = Note::from_str("Cbb3").unwrap();
        let flat_note = Note::from_str("Cb3").unwrap();
        let natural_note = Note::from_str("C3").unwrap();
        let sharp_note = Note::from_str("C#3").unwrap();
        let doublesharp_note = Note::from_str("C##3").unwrap();

        assert_eq!(natural_note.set_accidental(Accidental::Doubleflat), doubleflat_note);
        assert_eq!(natural_note.set_accidental(Accidental::Flat), flat_note);
        assert_eq!(natural_note.set_accidental(Accidental::Natural), natural_note);
        assert_eq!(natural_note.set_accidental(Accidental::Sharp), sharp_note);
        assert_eq!(natural_note.set_accidental(Accidental::Doublesharp), doublesharp_note);
    }

    #[test]
    fn next_natural() {
        assert_eq!(Note::from_str("Cbb3").unwrap().next_natural(), Note::from_str("D3").unwrap());
        assert_eq!(Note::from_str("Db3").unwrap().next_natural(),  Note::from_str("E3").unwrap());
        assert_eq!(Note::from_str("E3").unwrap().next_natural(),   Note::from_str("F3").unwrap());
        assert_eq!(Note::from_str("F#3").unwrap().next_natural(),  Note::from_str("G3").unwrap());
        assert_eq!(Note::from_str("G##3").unwrap().next_natural(), Note::from_str("A3").unwrap());
        assert_eq!(Note::from_str("A#3").unwrap().next_natural(),  Note::from_str("B3").unwrap());
        assert_eq!(Note::from_str("B3").unwrap().next_natural(),   Note::from_str("C4").unwrap());
        assert_eq!(Note::from_str("Cb4").unwrap().next_natural(),  Note::from_str("D4").unwrap());
    }

    #[test]
    fn prev_natural() {
        assert_eq!(Note::from_str("Cbb3").unwrap().prev_natural(), Note::from_str("B2").unwrap());
        assert_eq!(Note::from_str("Db3").unwrap().prev_natural(),  Note::from_str("C3").unwrap());
        assert_eq!(Note::from_str("E3").unwrap().prev_natural(),   Note::from_str("D3").unwrap());
        assert_eq!(Note::from_str("F#3").unwrap().prev_natural(),  Note::from_str("E3").unwrap());
        assert_eq!(Note::from_str("G##3").unwrap().prev_natural(), Note::from_str("F3").unwrap());
        assert_eq!(Note::from_str("A#3").unwrap().prev_natural(),  Note::from_str("G3").unwrap());
        assert_eq!(Note::from_str("B3").unwrap().prev_natural(),   Note::from_str("A3").unwrap());
        assert_eq!(Note::from_str("Cb4").unwrap().prev_natural(),  Note::from_str("B3").unwrap());
    }
}
