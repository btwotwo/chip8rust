use crossterm::KeyEvent;
use std::collections::HashMap;

macro_rules! key_map {
    ($($key:expr => $val: expr), *) => {
        {
            let mut map = HashMap::<KeyEvent, u8>::new();
            $(
                map.insert(KeyEvent::Char($key), $val);
            )*
            map
        }
    };
}

pub struct Keyboard {
    keys: [bool; 0xF],
    mapping: HashMap<KeyEvent, u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            keys: [false; 0xF],
            mapping: key_map!(
                '1' => 0x1,
                '2' => 0x2,
                '3' => 0x3,
                '4' => 0xC,
                'q' => 0x4,
                'w' => 0x5,
                'e' => 0x6,
                'r' => 0xD,
                'a' => 0x7,
                's' => 0x8,
                'd' => 0x9,
                'f' => 0xe,
                'z' => 0xA,
                'x' => 0,
                'c' => 0xB,
                'v' => 0xF
            ),
        }
    }

    pub fn set_state(&mut self, index: u8, state: bool) {
        self.keys[index as usize] = state;
    }

    pub fn get_state(&self, index: u8) -> bool {
        self.keys[index as usize]
    }

    pub fn press(&mut self, key: KeyEvent, state: bool) {
        if self.mapping.contains_key(&key) {
            let index = self.mapping[&key];
            self.keys[index as usize] = state
        }
    }
}
