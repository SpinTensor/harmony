use crate::notenames::NoteName;
use crate::accidentals::Accidental;

#[derive(Debug, PartialEq)]
pub struct Note {
    name: NoteName,
    accidental: Accidental,
    octave: u8,
}
