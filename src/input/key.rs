use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct KeysRes(pub HashMap<Key, bool>);

impl KeysRes {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn pressed(&self, key: Key) -> bool {
        match self.0.contains_key(&key) {
            true => self.0[&key],
            false => false
        }
    }

    pub fn press(&mut self, key: Key) {
        self.0.insert(key, true);
    }

    pub fn release(&mut self, key: Key) {
        self.0.insert(key, false);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Key {
    Unknown,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Minus,
    Equals,
    OpenBracket,
    CloseBracket,
    Backslash,
    Semicolon,
    Apostrophe,
    Comma,
    Dot,
    Slash,
    Backtick,
    Backspace,
    Tab,
    Return,
    Shift,
    Control,
    Opt,
    Space,
    Up,
    Down,
    Left,
    Right
}

impl Key {
    pub fn to_char(&self, shift: bool) -> Option<char> {
        match self {
            Key::A if !shift => Some('a'),
            Key::B if !shift => Some('b'),
            Key::C if !shift => Some('c'),
            Key::D if !shift => Some('d'),
            Key::E if !shift => Some('e'),
            Key::F if !shift => Some('f'),
            Key::G if !shift => Some('g'),
            Key::H if !shift => Some('h'),
            Key::I if !shift => Some('i'),
            Key::J if !shift => Some('j'),
            Key::K if !shift => Some('k'),
            Key::L if !shift => Some('l'),
            Key::M if !shift => Some('m'),
            Key::N if !shift => Some('n'),
            Key::O if !shift => Some('o'),
            Key::P if !shift => Some('p'),
            Key::Q if !shift => Some('q'),
            Key::R if !shift => Some('r'),
            Key::S if !shift => Some('s'),
            Key::T if !shift => Some('t'),
            Key::U if !shift => Some('u'),
            Key::V if !shift => Some('v'),
            Key::W if !shift => Some('w'),
            Key::X if !shift => Some('x'),
            Key::Y if !shift => Some('y'),
            Key::Z if !shift => Some('z'),
            Key::One if !shift => Some('1'),
            Key::Two if !shift => Some('2'),
            Key::Three if !shift => Some('3'),
            Key::Four if !shift => Some('4'),
            Key::Five if !shift => Some('5'),
            Key::Six if !shift => Some('6'),
            Key::Seven if !shift => Some('7'),
            Key::Eight if !shift => Some('8'),
            Key::Nine if !shift => Some('9'),
            Key::Zero if !shift => Some('0'),
            Key::Minus if !shift => Some('-'),
            Key::Equals if !shift => Some('='),
            Key::OpenBracket if !shift => Some('['),
            Key::CloseBracket if !shift => Some(']'),
            Key::Backslash if !shift => Some('\\'),
            Key::Semicolon if !shift => Some(';'),
            Key::Apostrophe if !shift => Some('\''),
            Key::Comma if !shift => Some(','),
            Key::Dot if !shift => Some('.'),
            Key::Slash if !shift => Some('/'),
            Key::Backtick if !shift => Some('`'),
            Key::A if shift => Some('A'),
            Key::B if shift => Some('B'),
            Key::C if shift => Some('C'),
            Key::D if shift => Some('D'),
            Key::E if shift => Some('E'),
            Key::F if shift => Some('F'),
            Key::G if shift => Some('G'),
            Key::H if shift => Some('H'),
            Key::I if shift => Some('I'),
            Key::J if shift => Some('J'),
            Key::K if shift => Some('K'),
            Key::L if shift => Some('L'),
            Key::M if shift => Some('M'),
            Key::N if shift => Some('N'),
            Key::O if shift => Some('O'),
            Key::P if shift => Some('P'),
            Key::Q if shift => Some('Q'),
            Key::R if shift => Some('R'),
            Key::S if shift => Some('S'),
            Key::T if shift => Some('T'),
            Key::U if shift => Some('U'),
            Key::V if shift => Some('V'),
            Key::W if shift => Some('W'),
            Key::X if shift => Some('X'),
            Key::Y if shift => Some('Y'),
            Key::Z if shift => Some('Z'),
            Key::One if shift => Some('!'),
            Key::Two if shift => Some('@'),
            Key::Three if shift => Some('#'),
            Key::Four if shift => Some('$'),
            Key::Five if shift => Some('%'),
            Key::Six if shift => Some('^'),
            Key::Seven if shift => Some('&'),
            Key::Eight if shift => Some('*'),
            Key::Nine if shift => Some('('),
            Key::Zero if shift => Some(')'),
            Key::Minus if shift => Some('_'),
            Key::Equals if shift => Some('+'),
            Key::OpenBracket if shift => Some('{'),
            Key::CloseBracket if shift => Some('{'),
            Key::Backslash if shift => Some('|'),
            Key::Semicolon if shift => Some(':'),
            Key::Apostrophe if shift => Some('"'),
            Key::Comma if shift => Some('<'),
            Key::Dot if shift => Some('>'),
            Key::Slash if shift => Some('?'),
            Key::Backtick if shift => Some('~'),
            Key::Space => Some(' '),
            _ => None
        }
    }
}
