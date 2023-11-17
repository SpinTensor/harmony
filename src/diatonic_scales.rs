use std::cmp::Ordering;
use crate::notes::Note;
use crate::accidentals::Accidental;
use crate::notesequences::NoteSequence;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl Mode {
    fn identify(noteseq: &NoteSequence) -> Result<Self, &'static str> {
        match noteseq.notes.len().cmp(&7) {
            Ordering::Less => return Err("Note sequence to short! Must be seven notes long!"),
            Ordering::Equal => (),
            Ordering::Greater => return Err("Note sequence to long! Must be seven notes long!"),
        }
        let steps: Vec<i32> = noteseq.notes.windows(2).map(|x| x[0].dist_hsteps(&x[1])).collect();
        for mode in [Mode::Ionian, Mode::Dorian, Mode::Phrygian, Mode::Lydian,
                     Mode::Mixolydian, Mode::Aeolian, Mode::Locrian] {
            if steps == mode.get_dists() {return Ok(mode);}
        }
        Err("Unknown diatonic note sequence mode")
    }

    fn get_dists(self) -> Vec<i32> {
        let mut dists = vec![2,2,1,2,2,2,1];
        match self {
            Mode::Ionian =>     dists.rotate_left(0), 
            Mode::Dorian =>     dists.rotate_left(1), 
            Mode::Phrygian =>   dists.rotate_left(2), 
            Mode::Lydian =>     dists.rotate_left(3), 
            Mode::Mixolydian => dists.rotate_left(4),
            Mode::Aeolian =>    dists.rotate_left(5),
            Mode::Locrian =>    dists.rotate_left(6),
        }
        dists[..6].to_vec()
    }
}

#[derive(Debug, PartialEq)]
pub struct DiatonicScale {
    tonic: Note,
    mode: Mode,
    notesequence: NoteSequence,
}

impl DiatonicScale {
    pub fn from_tonic(tonic: Note, mode: Mode) -> DiatonicScale {
        let mut scale = DiatonicScale {
            tonic, mode: Mode::Ionian,
            notesequence: NoteSequence::empty(),
        };
        scale.notesequence.notes.push(tonic);
        for inote in 1..7 {
            scale.notesequence.notes.push(tonic.shift_natural(inote));
        }

        let dists = mode.get_dists();
        for inote in 0..scale.notesequence.notes.len()-1 {
            scale.notesequence.notes[inote+1] = 
                match scale.notesequence.notes[inote].dist_hsteps(&scale.notesequence.notes[inote+1])-dists[inote] {
                    -2 => scale.notesequence.notes[inote+1].set_accidental(Accidental::Doublesharp),
                    -1 => scale.notesequence.notes[inote+1].set_accidental(Accidental::Sharp),
                     0 => scale.notesequence.notes[inote+1].set_accidental(Accidental::Natural),
                     1 => scale.notesequence.notes[inote+1].set_accidental(Accidental::Flat),
                     2 => scale.notesequence.notes[inote+1].set_accidental(Accidental::Doubleflat),
                    _ => panic!("unknown accidental required"),
            };
        }
        scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_dists() {
        assert_eq!(Mode::Ionian.get_dists(),     vec![2,2,1,2,2,2]);
        assert_eq!(Mode::Dorian.get_dists(),     vec![2,1,2,2,2,1]);
        assert_eq!(Mode::Phrygian.get_dists(),   vec![1,2,2,2,1,2]);
        assert_eq!(Mode::Lydian.get_dists(),     vec![2,2,2,1,2,2]);
        assert_eq!(Mode::Mixolydian.get_dists(), vec![2,2,1,2,2,1]);
        assert_eq!(Mode::Aeolian.get_dists(),    vec![2,1,2,2,1,2]);
        assert_eq!(Mode::Locrian.get_dists(),    vec![1,2,2,1,2,2]);
    }

    #[test]
    fn identify_diatonics() {
        let ionian_notesequence = NoteSequence::from_strs(["C3","D3","E3","F3","G3","A3","B3"].to_vec()).unwrap();
        let dorian_notesequence = NoteSequence::from_strs(["C3","D3","Eb3","F3","G3","A3","Bb3"].to_vec()).unwrap();
        let phrygian_notesequence = NoteSequence::from_strs(["C3","Db3","Eb3","F3","G3","Ab3","Bb3"].to_vec()).unwrap();
        let lydian_notesequence = NoteSequence::from_strs(["C3","D3","E3","F#3","G3","A3","B3"].to_vec()).unwrap();
        let mixolydian_notesequence = NoteSequence::from_strs(["C3","D3","E3","F3","G3","A3","Bb3"].to_vec()).unwrap();
        let aeolian_notesequence = NoteSequence::from_strs(["C3","D3","Eb3","F3","G3","Ab3","Bb3"].to_vec()).unwrap();
        let locrian_notesequence = NoteSequence::from_strs(["C3","Db3","Eb3","F3","Gb3","Ab3","Bb3"].to_vec()).unwrap();

        assert_eq!(Mode::identify(&ionian_notesequence), Ok(Mode::Ionian));
        assert_eq!(Mode::identify(&dorian_notesequence), Ok(Mode::Dorian));
        assert_eq!(Mode::identify(&phrygian_notesequence), Ok(Mode::Phrygian));
        assert_eq!(Mode::identify(&lydian_notesequence), Ok(Mode::Lydian));
        assert_eq!(Mode::identify(&mixolydian_notesequence), Ok(Mode::Mixolydian));
        assert_eq!(Mode::identify(&aeolian_notesequence), Ok(Mode::Aeolian));
        assert_eq!(Mode::identify(&locrian_notesequence), Ok(Mode::Locrian));

        let ionian_notesequence = NoteSequence::from_strs(["C3","D3","E3","F3","G3","A3","B3"].to_vec()).unwrap();
        let dorian_notesequence = NoteSequence::from_strs(["D3","E3","F3","G3","A3","B3","C4"].to_vec()).unwrap();
        let phrygian_notesequence = NoteSequence::from_strs(["E3","F3","G3","A3","B3","C4","D4"].to_vec()).unwrap();
        let lydian_notesequence = NoteSequence::from_strs(["F3","G3","A3","B3","C4","D4","E4"].to_vec()).unwrap();
        let mixolydian_notesequence = NoteSequence::from_strs(["G3","A3","B3","C4","D4","E4","F4"].to_vec()).unwrap();
        let aeolian_notesequence = NoteSequence::from_strs(["A3","B3","C4","D4","E4","F4","G4"].to_vec()).unwrap();
        let locrian_notesequence = NoteSequence::from_strs(["B3","C4","D4","E4","F4","G4","A4"].to_vec()).unwrap();

        assert_eq!(Mode::identify(&ionian_notesequence), Ok(Mode::Ionian));
        assert_eq!(Mode::identify(&dorian_notesequence), Ok(Mode::Dorian));
        assert_eq!(Mode::identify(&phrygian_notesequence), Ok(Mode::Phrygian));
        assert_eq!(Mode::identify(&lydian_notesequence), Ok(Mode::Lydian));
        assert_eq!(Mode::identify(&mixolydian_notesequence), Ok(Mode::Mixolydian));
        assert_eq!(Mode::identify(&aeolian_notesequence), Ok(Mode::Aeolian));
        assert_eq!(Mode::identify(&locrian_notesequence), Ok(Mode::Locrian));
    }

    #[test]
    fn new_ionian() {
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("Cb3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("Cb3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["Cb3", "Db3", "Eb3", "Fb3", "Gb3", "Ab3", "Bb3"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("C3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("C3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["C3", "D3", "E3", "F3", "G3", "A3", "B3"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("C#3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("C#3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["C#3", "D#3", "E#3", "F#3", "G#3", "A#3", "B#3"].to_vec()).unwrap()});

        assert_eq!(DiatonicScale::from_tonic(Note::from_str("Db3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("Db3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["Db3", "Eb3", "F3", "Gb3", "Ab3", "Bb3", "C4"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("D3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("D3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["D3", "E3", "F#3", "G3", "A3", "B3", "C#4"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("D#3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("D#3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["D#3", "E#3", "F##3", "G#3", "A#3", "B#3", "C##4"].to_vec()).unwrap()});
    
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("Eb3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("Eb3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["Eb3", "F3", "G3", "Ab3", "Bb3", "C4", "D4"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("E3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("E3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["E3", "F#3", "G#3", "A3", "B3", "C#4", "D#4"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("E#3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("E#3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["E#3", "F##3", "G##3", "A#3", "B#3", "C##4", "D##4"].to_vec()).unwrap()});

        assert_eq!(DiatonicScale::from_tonic(Note::from_str("Fb3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("Fb3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["Fb3", "Gb3", "Ab3", "Bbb3", "Cb4", "Db4", "Eb4"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("F3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("F3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["F3", "G3", "A3", "Bb3", "C4", "D4", "E4"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("F#3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("F#3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["F#3", "G#3", "A#3", "B3", "C#4", "D#4", "E#4"].to_vec()).unwrap()});

        assert_eq!(DiatonicScale::from_tonic(Note::from_str("Gb3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("Gb3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["Gb3", "Ab3", "Bb3", "Cb4", "Db4", "Eb4", "F4"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("G3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("G3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["G3", "A3", "B3", "C4", "D4", "E4", "F#4"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("G#3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("G#3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["G#3", "A#3", "B#3", "C#4", "D#4", "E#4", "F##4"].to_vec()).unwrap()});

        assert_eq!(DiatonicScale::from_tonic(Note::from_str("Ab3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("Ab3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["Ab3", "Bb3", "C4", "Db4", "Eb4", "F4", "G4"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("A3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("A3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["A3", "B3", "C#4", "D4", "E4", "F#4", "G#4"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("A#3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("A#3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["A#3", "B#3", "C##4", "D#4", "E#4", "F##4", "G##4"].to_vec()).unwrap()});

        assert_eq!(DiatonicScale::from_tonic(Note::from_str("Bb3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("Bb3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["Bb3", "C4", "D4", "Eb4", "F4", "G4", "A4"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("B3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("B3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["B3", "C#4", "D#4", "E4", "F#4", "G#4", "A#4"].to_vec()).unwrap()});
        assert_eq!(DiatonicScale::from_tonic(Note::from_str("B#3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("B#3").unwrap(), mode: Mode::Ionian,
                   notesequence: NoteSequence::from_strs(["B#3", "C##4", "D##4", "E#4", "F##4", "G##4", "A##4"].to_vec()).unwrap()});
    }
}
