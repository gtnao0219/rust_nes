use crate::Byte;

#[derive(Debug)]
pub struct Controller {
    key_state: [bool; 8],
    is_set: bool,
    index: usize,
    register: [bool; 8],
}

impl Default for Controller {
    fn default() -> Self {
        Controller {
            key_state: [false; 8],
            is_set: false,
            index: 0,
            register: [false; 8],
        }
    }
}

impl Controller {
    pub fn read(&mut self) -> bool {
        let result = self.register[self.index];
        self.index = (self.index + 1) % 8;
        result
    }
    pub fn write(&mut self, data: Byte) {
        if data & 0x01 != 0 {
            self.is_set = true;
        } else if self.is_set {
            self.is_set = false;
            self.index = 0;
            for i in 0..8 {
                self.register[i] = self.key_state[i];
            }
        }
    }
    // A = 0,
    // B = 1,
    // Select = 2,
    // Start = 3,
    // Up = 4,
    // Down = 5,
    // Left = 6,
    // Right = 7,
    pub fn key_down(&mut self, key: u8) {
        self.key_state[key as usize] = true;
    }
    pub fn key_up(&mut self, key: u8) {
        self.key_state[key as usize] = false;
    }
}
