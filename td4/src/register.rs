#[derive(Debug)]
pub struct Register {
    pc: u8,
    c: u8,
    a: u8,
    b: u8,
}

impl Register {
    pub fn new() -> Self {
        return Self {
            pc: 0,
            c: 0,
            a: 0,
            b: 0,
        };
    }

    // Program Counter
    pub fn get_pc(&self) -> u8 {
        return self.pc;
    }

    pub fn set_pc(&mut self, v: u8) {
        self.pc = v;
    }

    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }

    // Carry Flag
    pub fn get_c(&self) -> u8 {
        return self.c;
    }

    pub fn set_c(&mut self, v: u8) {
        self.c = v;
    }

    // General Registers
    pub fn get_a(&self) -> u8 {
        return self.a;
    }

    pub fn set_a(&mut self, v: u8) {
        self.a = v;
    }

    pub fn get_b(&self) -> u8 {
        return self.b;
    }

    pub fn set_b(&mut self, v: u8) {
        self.b = v;
    }
}

#[cfg(test)]
mod tests {
    use crate::register::*;

    #[test]
    fn test_setter_and_getter() {
        let mut register = Register::new();

        assert_eq!(register.get_pc(), 0);
        register.set_pc(1);
        assert_eq!(register.get_pc(), 1);

        assert_eq!(register.get_c(), 0);
        register.set_c(1);
        assert_eq!(register.get_c(), 1);

        assert_eq!(register.get_a(), 0);
        register.set_a(1);
        assert_eq!(register.get_a(), 1);

        assert_eq!(register.get_b(), 0);
        register.set_b(1);
        assert_eq!(register.get_b(), 1);
    }

    #[test]
    fn test_increment_pc() {
        let mut register = Register::new();

        assert_eq!(register.get_pc(), 0);
        register.increment_pc();
        assert_eq!(register.get_pc(), 1);
        register.increment_pc();
        assert_eq!(register.get_pc(), 2);
    }
}
