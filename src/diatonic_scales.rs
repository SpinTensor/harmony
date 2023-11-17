use std::cmp::Ordering;
use crate::notes::Note;
use crate::accidentals::Accidental;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl Mode {
    fn identify(notes: Vec<Note>) -> Result<Self, &'static str> {
        match notes.len().cmp(&7) {
            Ordering::Less => return Err("Note sequence to short! Must be seven notes long!"),
            Ordering::Equal => (),
            Ordering::Greater => return Err("Note sequence to long! Must be seven notes long!"),
        }
        let steps: Vec<i32> = notes.windows(2).map(|x| x[0].dist_hsteps(&x[1])).collect();
        match steps[..] {
            [2,2,1,2,2,2] => Ok(Mode::Ionian),
            [2,1,2,2,2,1] => Ok(Mode::Dorian),
            [1,2,2,2,1,2] => Ok(Mode::Phrygian),
            [2,2,2,1,2,2] => Ok(Mode::Lydian),
            [2,2,1,2,2,1] => Ok(Mode::Mixolydian),
            [2,1,2,2,1,2] => Ok(Mode::Aeolian),
            [1,2,2,1,2,2] => Ok(Mode::Locrian),
            _ => Err("Unknown Note sequence mode"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct DiatonicScale {
    tonic: Note,
    mode: Mode,
    notes: Vec<Note>,
}

impl DiatonicScale {
    pub fn new(tonic: &Note, mode: Mode) -> DiatonicScale {
        match mode {
            Mode::Ionian => Self::new_ionian(tonic),
            _ => panic!("ASDFSADF")
        }
    }

    fn new_ionian(tonic: &Note) -> DiatonicScale {
        let mut scale = DiatonicScale {
            tonic: *tonic,
            mode: Mode::Ionian,
            notes: Vec::<Note>::new(),
        };
        scale.notes.push(*tonic);
        for inote in 1..7 {
            scale.notes.push(tonic.shift_natural(inote));
        }

        let ionian_dists= vec![2,2,1,2,2,2];
        for inote in 0..scale.notes.len()-1 {
            println!("{:?}", scale.notes[inote].dist_hsteps(&scale.notes[inote+1]));
            scale.notes[inote+1] = match scale.notes[inote].dist_hsteps(&scale.notes[inote+1])-ionian_dists[inote] {
                -2 => scale.notes[inote+1].set_accidental(Accidental::Doublesharp),
                -1 => scale.notes[inote+1].set_accidental(Accidental::Sharp),
                 0 => scale.notes[inote+1].set_accidental(Accidental::Natural),
                 1 => scale.notes[inote+1].set_accidental(Accidental::Flat),
                 2 => scale.notes[inote+1].set_accidental(Accidental::Doubleflat),
                _ => panic!("asdf")
            };
        }
        println!("{:?}", scale.tonic);
        println!("{:?}", scale.mode);
        for i in &scale.notes {
            print!(" {:?}", i.to_str());
        }
        println!();
        scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identify_diatonics() {
        let ionian_notes: Vec<Note> = ["C3","D3","E3","F3","G3","A3","B3"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();
        let dorian_notes: Vec<Note> = ["C3","D3","Eb3","F3","G3","A3","Bb3"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();
        let phrygian_notes: Vec<Note> = ["C3","Db3","Eb3","F3","G3","Ab3","Bb3"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();
        let lydian_notes: Vec<Note> = ["C3","D3","E3","F#3","G3","A3","B3"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();
        let mixolydian_notes: Vec<Note> = ["C3","D3","E3","F3","G3","A3","Bb3"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();
        let aeolian_notes: Vec<Note> = ["C3","D3","Eb3","F3","G3","Ab3","Bb3"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();
        let locrian_notes: Vec<Note> = ["C3","Db3","Eb3","F3","Gb3","Ab3","Bb3"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();

        assert_eq!(Mode::identify(ionian_notes), Ok(Mode::Ionian));
        assert_eq!(Mode::identify(dorian_notes), Ok(Mode::Dorian));
        assert_eq!(Mode::identify(phrygian_notes), Ok(Mode::Phrygian));
        assert_eq!(Mode::identify(lydian_notes), Ok(Mode::Lydian));
        assert_eq!(Mode::identify(mixolydian_notes), Ok(Mode::Mixolydian));
        assert_eq!(Mode::identify(aeolian_notes), Ok(Mode::Aeolian));
        assert_eq!(Mode::identify(locrian_notes), Ok(Mode::Locrian));

        let ionian_notes: Vec<Note> =     ["C3","D3","E3","F3","G3","A3","B3"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();
        let dorian_notes: Vec<Note> =     ["D3","E3","F3","G3","A3","B3","C4"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();
        let phrygian_notes: Vec<Note> =   ["E3","F3","G3","A3","B3","C4","D4"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();
        let lydian_notes: Vec<Note> =     ["F3","G3","A3","B3","C4","D4","E4"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();
        let mixolydian_notes: Vec<Note> = ["G3","A3","B3","C4","D4","E4","F4"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();
        let aeolian_notes: Vec<Note> =    ["A3","B3","C4","D4","E4","F4","G4"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();
        let locrian_notes: Vec<Note> =    ["B3","C4","D4","E4","F4","G4","A4"]
            .map(|x| Note::from_str(x).unwrap()).to_vec();

        assert_eq!(Mode::identify(ionian_notes), Ok(Mode::Ionian));
        assert_eq!(Mode::identify(dorian_notes), Ok(Mode::Dorian));
        assert_eq!(Mode::identify(phrygian_notes), Ok(Mode::Phrygian));
        assert_eq!(Mode::identify(lydian_notes), Ok(Mode::Lydian));
        assert_eq!(Mode::identify(mixolydian_notes), Ok(Mode::Mixolydian));
        assert_eq!(Mode::identify(aeolian_notes), Ok(Mode::Aeolian));
        assert_eq!(Mode::identify(locrian_notes), Ok(Mode::Locrian));
    }

    #[test]
    fn new_ionian() {
        assert_eq!(DiatonicScale::new(&Note::from_str("C3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("C3").unwrap(),
                                  mode: Mode::Ionian,
                                  notes: vec![Note::from_str("C3").unwrap(),
                                              Note::from_str("D3").unwrap(),
                                              Note::from_str("E3").unwrap(),
                                              Note::from_str("F3").unwrap(),
                                              Note::from_str("G3").unwrap(),
                                              Note::from_str("A3").unwrap(),
                                              Note::from_str("B3").unwrap()]});
        assert_eq!(DiatonicScale::new(&Note::from_str("G3").unwrap(), Mode::Ionian),
                   DiatonicScale {tonic: Note::from_str("G3").unwrap(),
                                  mode: Mode::Ionian,
                                  notes: vec![Note::from_str("G3").unwrap(),
                                              Note::from_str("A3").unwrap(),
                                              Note::from_str("B3").unwrap(),
                                              Note::from_str("C4").unwrap(),
                                              Note::from_str("D4").unwrap(),
                                              Note::from_str("E4").unwrap(),
                                              Note::from_str("F#4").unwrap()]});

    }
}
