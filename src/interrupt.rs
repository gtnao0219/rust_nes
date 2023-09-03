#[derive(Debug)]
pub struct Interrupt {
    nmi: bool,
    irq: bool,
}
impl Default for Interrupt {
    fn default() -> Self {
        Interrupt {
            nmi: false,
            irq: false,
        }
    }
}
impl Interrupt {
    pub fn is_nmi(&self) -> bool {
        self.nmi
    }
    pub fn is_irq(&self) -> bool {
        self.irq
    }
    pub fn set_nmi(&mut self) {
        self.nmi = true;
    }
    pub fn clear_nmi(&mut self) {
        self.nmi = false;
    }
    pub fn set_irq(&mut self) {
        self.irq = true;
    }
    pub fn clear_irq(&mut self) {
        self.irq = false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interrupt() {
        let mut interrupt = Interrupt::default();
        assert_eq!(interrupt.is_nmi(), false);
        assert_eq!(interrupt.is_irq(), false);
        interrupt.set_nmi();
        assert_eq!(interrupt.is_nmi(), true);
        assert_eq!(interrupt.is_irq(), false);
        interrupt.clear_nmi();
        assert_eq!(interrupt.is_nmi(), false);
        assert_eq!(interrupt.is_irq(), false);
        interrupt.set_irq();
        assert_eq!(interrupt.is_nmi(), false);
        assert_eq!(interrupt.is_irq(), true);
        interrupt.clear_irq();
        assert_eq!(interrupt.is_nmi(), false);
        assert_eq!(interrupt.is_irq(), false);
    }
}
