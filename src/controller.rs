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
        self.index = self.index + 1;
        result
    }
    pub fn write(&mut self, data: u8) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller() {
        let mut controller = Controller::default();
        controller.key_down(0);
        controller.key_down(1);
        controller.key_down(2);
        controller.key_down(3);
        controller.key_down(4);
        controller.key_down(5);
        controller.key_down(6);
        controller.key_down(7);
        controller.write(0x01);
        controller.write(0x00);
        assert_eq!(controller.read(), true);
        assert_eq!(controller.read(), true);
        assert_eq!(controller.read(), true);
        assert_eq!(controller.read(), true);
        assert_eq!(controller.read(), true);
        assert_eq!(controller.read(), true);
        assert_eq!(controller.read(), true);
        assert_eq!(controller.read(), true);

        controller.key_up(0);
        controller.key_up(1);
        controller.key_up(2);
        controller.key_up(3);
        controller.key_up(4);
        controller.key_up(5);
        controller.key_up(6);
        controller.key_up(7);
        controller.write(0x01);
        controller.write(0x00);
        assert_eq!(controller.read(), false);
        assert_eq!(controller.read(), false);
        assert_eq!(controller.read(), false);
        assert_eq!(controller.read(), false);
        assert_eq!(controller.read(), false);
        assert_eq!(controller.read(), false);
        assert_eq!(controller.read(), false);
        assert_eq!(controller.read(), false);
    }
}
