#[derive(Debug, PartialEq)]
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

    pub fn to_str(&self) -> &str {
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

    pub fn same(&self) -> Self {
        match self {
            NoteName::C => NoteName::C,
            NoteName::D => NoteName::D,
            NoteName::E => NoteName::E,
            NoteName::F => NoteName::F,
            NoteName::G => NoteName::G,
            NoteName::A => NoteName::A,
            NoteName::B => NoteName::B,
        }
    }

    pub fn next(&self) -> Self {
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

    pub fn prev(&self) -> Self {
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

    pub fn up(&self, steps: u32) -> Self {
        match steps {
            0 => self.same(),
            _ => self.next().up(steps-1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(NoteName::from_str(&String::from("C" )).unwrap(), NoteName::C);
        assert_eq!(NoteName::from_str(&String::from("D" )).unwrap(), NoteName::D);
        assert_eq!(NoteName::from_str(&String::from("E" )).unwrap(), NoteName::E);
        assert_eq!(NoteName::from_str(&String::from("F" )).unwrap(), NoteName::F);
        assert_eq!(NoteName::from_str(&String::from("G" )).unwrap(), NoteName::G);
        assert_eq!(NoteName::from_str(&String::from("A" )).unwrap(), NoteName::A);
        assert_eq!(NoteName::from_str(&String::from("B" )).unwrap(), NoteName::B);

        assert_eq!(NoteName::from_str(&String::from("c" )).unwrap(), NoteName::C);
        assert_eq!(NoteName::from_str(&String::from("d" )).unwrap(), NoteName::D);
        assert_eq!(NoteName::from_str(&String::from("e" )).unwrap(), NoteName::E);
        assert_eq!(NoteName::from_str(&String::from("f" )).unwrap(), NoteName::F);
        assert_eq!(NoteName::from_str(&String::from("g" )).unwrap(), NoteName::G);
        assert_eq!(NoteName::from_str(&String::from("a" )).unwrap(), NoteName::A);
        assert_eq!(NoteName::from_str(&String::from("b" )).unwrap(), NoteName::B);
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
}
