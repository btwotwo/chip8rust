use crossterm::input;
use crossterm::{InputEvent, KeyEvent};
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

#[derive(Debug)]
pub struct Keyboard {
    keys: [bool; 0xF],
    pub mapping: HashMap<KeyEvent, u8>,
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

    pub fn is_pressed(&self, index: u8) -> bool {
        self.keys[index as usize]
    }

    pub fn press(&mut self, key: KeyEvent, state: bool) {
        if self.mapping.contains_key(&key) {
            let index = self.mapping[&key];
            self.keys[index as usize] = state
        }
    }

    pub fn wait_for_key(&self) -> u8 {
        let input = input();

        *input
            .read_sync()
            .find_map(|event| match event {
                InputEvent::Keyboard(event) => match event {
                    KeyEvent::Ctrl('c') => std::process::exit(0),
                    _ => self.mapping.get(&event),
                },
                _ => None,
            })
            .unwrap()
    }

    pub fn register_key_press(&mut self) {
        let input = input();

        match input.read_async().next() {
            Some(InputEvent::Keyboard(event)) => match event {
                KeyEvent::Ctrl('c') => std::process::exit(0),
                _ => match self.mapping.get(&event) {
                    Some(key) => self.keys[*key as usize] = true,
                    None => self.keys = [false; 0xf]
                },
            },

            _ => ()
        }
    }
}
