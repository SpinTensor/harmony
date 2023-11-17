#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NoteName {
    C, D, E, F, G, A, B
}

impl NoteName {
    pub fn from_str(s:&str) -> Result<Self, &'static str> {
        match s.to_uppercase().as_str() {
            "C" => Ok(NoteName::C),
            "D" => Ok(NoteName::D),
            "E" => Ok(NoteName::E),
            "F" => Ok(NoteName::F),
            "G" => Ok(NoteName::G),
            "A" => Ok(NoteName::A),
            "B" => Ok(NoteName::B),
            _ => Err("Invalid Note Name")
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            NoteName::C => "C",
            NoteName::D => "D",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::G => "G",
            NoteName::A => "A",
            NoteName::B => "B",
        }
    }

    pub fn next(self) -> Self {
        match self {
            NoteName::C => NoteName::D,
            NoteName::D => NoteName::E,
            NoteName::E => NoteName::F,
            NoteName::F => NoteName::G,
            NoteName::G => NoteName::A,
            NoteName::A => NoteName::B,
            NoteName::B => NoteName::C,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            NoteName::C => NoteName::B,
            NoteName::D => NoteName::C,
            NoteName::E => NoteName::D,
            NoteName::F => NoteName::E,
            NoteName::G => NoteName::F,
            NoteName::A => NoteName::G,
            NoteName::B => NoteName::A,
        }
    }

    pub fn up(self, steps: u32) -> Self {
        let mut up_note = self;
        for _ in 0..steps {
            up_note = up_note.next();
        }
        up_note
    }

    pub fn down(self, steps: u32) -> Self {
        let mut down_note = self;
        for _ in 0..steps {
            down_note = down_note.prev();
        }
        down_note
    }

    pub fn shift(self, steps: i32) -> Self {
        use std::cmp::Ordering;
        match steps.cmp(&0) {
            Ordering::Less => self.down(steps.unsigned_abs()),
            Ordering::Equal => self,
            Ordering::Greater => self.up(steps.unsigned_abs()),
        }
    }

    fn to_idx(self) -> i32 {
        match self {
            NoteName::C => 0,
            NoteName::D => 1,
            NoteName::E => 2,
            NoteName::F => 3,
            NoteName::G => 4,
            NoteName::A => 5,
            NoteName::B => 6,
        }
    }

    pub fn dist(self, other: &NoteName) -> i32 {
        let selfidx = self.to_idx();
        let otheridx = other.to_idx();
        otheridx - selfidx
    }

    fn to_hsteps_idx(self) -> i32 {
        match self {
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
            NoteName::A => 9,
            NoteName::B => 11,
        }
    }

    pub fn dist_hsteps(self, other: &NoteName) -> i32 {
        let selfidx = self.to_hsteps_idx();
        let otheridx = other.to_hsteps_idx();
        otheridx - selfidx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(NoteName::from_str(&String::from("C" )), Ok(NoteName::C));
        assert_eq!(NoteName::from_str(&String::from("D" )), Ok(NoteName::D));
        assert_eq!(NoteName::from_str(&String::from("E" )), Ok(NoteName::E));
        assert_eq!(NoteName::from_str(&String::from("F" )), Ok(NoteName::F));
        assert_eq!(NoteName::from_str(&String::from("G" )), Ok(NoteName::G));
        assert_eq!(NoteName::from_str(&String::from("A" )), Ok(NoteName::A));
        assert_eq!(NoteName::from_str(&String::from("B" )), Ok(NoteName::B));

        assert_eq!(NoteName::from_str(&String::from("c" )), Ok(NoteName::C));
        assert_eq!(NoteName::from_str(&String::from("d" )), Ok(NoteName::D));
        assert_eq!(NoteName::from_str(&String::from("e" )), Ok(NoteName::E));
        assert_eq!(NoteName::from_str(&String::from("f" )), Ok(NoteName::F));
        assert_eq!(NoteName::from_str(&String::from("g" )), Ok(NoteName::G));
        assert_eq!(NoteName::from_str(&String::from("a" )), Ok(NoteName::A));
        assert_eq!(NoteName::from_str(&String::from("b" )), Ok(NoteName::B));

        assert!(NoteName::from_str(&String::from("H")).is_err());
        assert!(NoteName::from_str(&String::from("h")).is_err());
        assert!(NoteName::from_str(&String::from("This should fail")).is_err());
    }

    #[test]
    fn to_str() {
        assert_eq!(NoteName::C.to_str(), String::from("C"));
        assert_eq!(NoteName::D.to_str(), String::from("D"));
        assert_eq!(NoteName::E.to_str(), String::from("E"));
        assert_eq!(NoteName::F.to_str(), String::from("F"));
        assert_eq!(NoteName::G.to_str(), String::from("G"));
        assert_eq!(NoteName::A.to_str(), String::from("A"));
        assert_eq!(NoteName::B.to_str(), String::from("B"));
    }

    #[test]
    fn next() {
        assert_eq!(NoteName::C.next(), NoteName::D);
        assert_eq!(NoteName::D.next(), NoteName::E);
        assert_eq!(NoteName::E.next(), NoteName::F);
        assert_eq!(NoteName::F.next(), NoteName::G);
        assert_eq!(NoteName::G.next(), NoteName::A);
        assert_eq!(NoteName::A.next(), NoteName::B);
        assert_eq!(NoteName::B.next(), NoteName::C);
    }

    #[test]
    fn prev() {
        assert_eq!(NoteName::C.prev(), NoteName::B);
        assert_eq!(NoteName::D.prev(), NoteName::C);
        assert_eq!(NoteName::E.prev(), NoteName::D);
        assert_eq!(NoteName::F.prev(), NoteName::E);
        assert_eq!(NoteName::G.prev(), NoteName::F);
        assert_eq!(NoteName::A.prev(), NoteName::G);
        assert_eq!(NoteName::B.prev(), NoteName::A);
    }

    #[test]
    fn up() {
        assert_eq!(NoteName::C.up(0), NoteName::C);
        assert_eq!(NoteName::C.up(1), NoteName::D);
        assert_eq!(NoteName::C.up(2), NoteName::E);
        assert_eq!(NoteName::C.up(3), NoteName::F);
        assert_eq!(NoteName::C.up(4), NoteName::G);
        assert_eq!(NoteName::C.up(5), NoteName::A);
        assert_eq!(NoteName::C.up(6), NoteName::B);
        assert_eq!(NoteName::C.up(7), NoteName::C);
        assert_eq!(NoteName::C.up(8), NoteName::D);
    }

    #[test]
    fn down() {
        assert_eq!(NoteName::C.down(0), NoteName::C);
        assert_eq!(NoteName::C.down(1), NoteName::B);
        assert_eq!(NoteName::C.down(2), NoteName::A);
        assert_eq!(NoteName::C.down(3), NoteName::G);
        assert_eq!(NoteName::C.down(4), NoteName::F);
        assert_eq!(NoteName::C.down(5), NoteName::E);
        assert_eq!(NoteName::C.down(6), NoteName::D);
        assert_eq!(NoteName::C.down(7), NoteName::C);
        assert_eq!(NoteName::C.down(8), NoteName::B);
    }

    #[test]
    fn shift() {
        assert_eq!(NoteName::C.shift(-8), NoteName::B);
        assert_eq!(NoteName::C.shift(-7), NoteName::C);
        assert_eq!(NoteName::C.shift(-6), NoteName::D);
        assert_eq!(NoteName::C.shift(-5), NoteName::E);
        assert_eq!(NoteName::C.shift(-4), NoteName::F);
        assert_eq!(NoteName::C.shift(-3), NoteName::G);
        assert_eq!(NoteName::C.shift(-2), NoteName::A);
        assert_eq!(NoteName::C.shift(-1), NoteName::B);
        assert_eq!(NoteName::C.shift(0), NoteName::C);
        assert_eq!(NoteName::C.shift(1), NoteName::D);
        assert_eq!(NoteName::C.shift(2), NoteName::E);
        assert_eq!(NoteName::C.shift(3), NoteName::F);
        assert_eq!(NoteName::C.shift(4), NoteName::G);
        assert_eq!(NoteName::C.shift(5), NoteName::A);
        assert_eq!(NoteName::C.shift(6), NoteName::B);
        assert_eq!(NoteName::C.shift(7), NoteName::C);
        assert_eq!(NoteName::C.shift(8), NoteName::D);
    }

    #[test]
    fn dist() {
        assert_eq!(NoteName::C.dist(&NoteName::C), 0);
        assert_eq!(NoteName::C.dist(&NoteName::D), 1);
        assert_eq!(NoteName::C.dist(&NoteName::E), 2);
        assert_eq!(NoteName::C.dist(&NoteName::F), 3);
        assert_eq!(NoteName::C.dist(&NoteName::G), 4);
        assert_eq!(NoteName::C.dist(&NoteName::A), 5);
        assert_eq!(NoteName::C.dist(&NoteName::B), 6);

        assert_eq!(NoteName::C.dist(&NoteName::C), 0);
        assert_eq!(NoteName::D.dist(&NoteName::C), -1);
        assert_eq!(NoteName::E.dist(&NoteName::C), -2);
        assert_eq!(NoteName::F.dist(&NoteName::C), -3);
        assert_eq!(NoteName::G.dist(&NoteName::C), -4);
        assert_eq!(NoteName::A.dist(&NoteName::C), -5);
        assert_eq!(NoteName::B.dist(&NoteName::C), -6);
    }

    #[test]
    fn dist_hsteps() {
        assert_eq!(NoteName::C.dist_hsteps(&NoteName::C), 0);
        assert_eq!(NoteName::C.dist_hsteps(&NoteName::D), 2);
        assert_eq!(NoteName::C.dist_hsteps(&NoteName::E), 4);
        assert_eq!(NoteName::C.dist_hsteps(&NoteName::F), 5);
        assert_eq!(NoteName::C.dist_hsteps(&NoteName::G), 7);
        assert_eq!(NoteName::C.dist_hsteps(&NoteName::A), 9);
        assert_eq!(NoteName::C.dist_hsteps(&NoteName::B), 11);

        assert_eq!(NoteName::C.dist_hsteps(&NoteName::C), 0);
        assert_eq!(NoteName::D.dist_hsteps(&NoteName::C), -2);
        assert_eq!(NoteName::E.dist_hsteps(&NoteName::C), -4);
        assert_eq!(NoteName::F.dist_hsteps(&NoteName::C), -5);
        assert_eq!(NoteName::G.dist_hsteps(&NoteName::C), -7);
        assert_eq!(NoteName::A.dist_hsteps(&NoteName::C), -9);
        assert_eq!(NoteName::B.dist_hsteps(&NoteName::C), -11);
    }
}
