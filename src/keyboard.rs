use crossterm::event::KeyModifiers;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

use std::collections::HashMap;

macro_rules! key_map {
    ($($key:expr => $val: expr), *) => {
        {
            let mut map = HashMap::<KeyCode, u8>::new();
            $(
                map.insert(KeyCode::Char($key), $val);
            )*
            map
        }
    };
}

#[derive(Debug)]
pub struct Keyboard {
    keys: [bool; 0xF + 1],
    pub mapping: HashMap<KeyCode, u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            keys: [false; 0xF + 1],
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

    pub fn press(&mut self, key: KeyCode, state: bool) {
        if self.mapping.contains_key(&key) {
            let index = self.mapping[&key];
            self.keys[index as usize] = state
        }
    }

    pub fn wait_for_key(&self) -> u8 {
        loop {
            match read().unwrap() {
                Event::Key(event) => {
                    if event.modifiers.contains(KeyModifiers::CONTROL)
                        && event.code == KeyCode::Char('c')
                    {
                        std::process::exit(0)
                    } else {
                        let key = self.mapping.get(&event.code);
                        if let Some(key) = key {
                            return key.clone();
                        } else {
                            continue;
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    pub fn register_key_press(&mut self) {
        use std::time::Duration;

        if poll(Duration::from_millis(5)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => {
                    if event.modifiers.contains(KeyModifiers::CONTROL)
                        && event.code == KeyCode::Char('c')
                    {
                        std::process::exit(0)
                    } else {
                        match self.mapping.get(&event.code) {
                            Some(key) => self.keys[*key as usize] = true,
                            None => self.keys = [false; 0xf + 1],
                        }
                    }
                },
                _ => ()
            }
        }
    }
}
