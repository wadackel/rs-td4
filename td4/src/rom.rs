// ROM
#[derive(Debug)]
pub struct Rom {
    vec: Vec<u8>,
}

impl Rom {
    pub fn new(buf: Vec<u8>) -> Self {
        return Self { vec: buf.clone() };
    }

    pub fn read(&self, addr: u8) -> u8 {
        return self.vec[addr as usize];
    }

    pub fn size(&self) -> u8 {
        return self.vec.len() as u8;
    }
}

#[cfg(test)]
mod tests {
    use crate::rom::*;

    #[test]
    fn test_rom() {
        let rom = Rom::new(vec![0, 0, 1, 0]);

        assert_eq!(rom.size(), 4);
        assert_eq!(rom.read(0), 0);
        assert_eq!(rom.read(1), 0);
        assert_eq!(rom.read(2), 1);
        assert_eq!(rom.read(3), 0);
    }
}
