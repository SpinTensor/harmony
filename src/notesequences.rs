use crate::notes::Note;

#[derive(Debug, PartialEq, Clone)]
pub struct NoteSequence {
    pub notes: Vec<Note>
}

impl NoteSequence {
    pub fn empty() -> Self {
        Self {
            notes: Vec::<Note>::new()
        }
    }

    pub fn from_strs(svec: Vec<&str>) -> Result<Self, &'static str> {
        let mut sequence: Self = NoteSequence::empty();
        for note in svec.iter().map(|x| Note::from_str(x)) {
            match note {
                Ok(note) => sequence.notes.push(note),
                Err(err) => return Err(err),
            }
        }
        Ok(sequence)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_strs() {
        let strs = [];
        assert_eq!(NoteSequence::from_strs(strs.to_vec()), Ok(NoteSequence {notes: strs.map(|x| Note::from_str(x).unwrap()).to_vec()}));
        let strs = ["C3"];
        assert_eq!(NoteSequence::from_strs(strs.to_vec()), Ok(NoteSequence {notes: strs.map(|x| Note::from_str(x).unwrap()).to_vec()}));
        let strs = ["C3","D3","E3"];
        assert_eq!(NoteSequence::from_strs(strs.to_vec()), Ok(NoteSequence {notes: strs.map(|x| Note::from_str(x).unwrap()).to_vec()}));
        let strs = ["C3","D3","E3","F3","G3","A3","B3"];
        assert_eq!(NoteSequence::from_strs(strs.to_vec()), Ok(NoteSequence {notes: strs.map(|x| Note::from_str(x).unwrap()).to_vec()}));
        let strs = ["C2","D#3","E##3","F3","Gb3","Abb-3","B##-3"];
        assert_eq!(NoteSequence::from_strs(strs.to_vec()), Ok(NoteSequence {notes: strs.map(|x| Note::from_str(x).unwrap()).to_vec()}));

        let strs = ["C3","D3","E3","F3","G#","A3","B3"];
        assert!(NoteSequence::from_strs(strs.to_vec()).is_err());
    }
}
