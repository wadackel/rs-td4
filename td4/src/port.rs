// PORT
#[derive(Debug)]
pub struct Port {
    input: u8,
    output: u8,
}

impl Port {
    pub fn new() -> Self {
        return Self {
            input: 0,
            output: 0,
        };
    }

    pub fn set_input(&mut self, v: u8) {
        self.input = v;
    }

    pub fn get_input(&self) -> u8 {
        return self.input;
    }

    pub fn set_output(&mut self, v: u8) {
        self.output = v;
    }

    pub fn get_output(&self) -> u8 {
        return self.output;
    }
}

#[cfg(test)]
mod tests {
    use crate::port::*;

    #[test]
    fn test_setter_and_getter() {
        let mut port = Port::new();

        assert_eq!(port.get_output(), 0);
        port.set_output(1);
        assert_eq!(port.get_output(), 1);

        assert_eq!(port.get_input(), 0);
        port.set_input(1);
        assert_eq!(port.get_input(), 1);
    }
}
