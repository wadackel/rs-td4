use crate::opcode::*;
use crate::port::*;
use crate::register::*;
use crate::rom::*;

fn fetch(register: &mut Register, rom: Rom) -> u8 {
    let pc = register.get_pc();
    if rom.size() <= pc {
        return 0;
    }

    let code = rom.read(pc);

    register.increment_pc();

    return code;
}

fn decode(data: u8) -> (Opcode, u8) {
    let op = data >> 4;
    let im = data & 0x0f;

    return match num_traits::FromPrimitive::from_u8(op) {
        Some(Opcode::AddA) => (Opcode::AddA, im),
        Some(Opcode::AddB) => (Opcode::AddB, im),
        Some(Opcode::MovA) => (Opcode::MovA, im),
        Some(Opcode::MovB) => (Opcode::MovB, im),
        Some(Opcode::MovAB) => (Opcode::MovAB, im),
        Some(Opcode::MovBA) => (Opcode::MovBA, im),
        Some(Opcode::Jmp) => (Opcode::Jmp, im),
        Some(Opcode::Jnc) => (Opcode::Jnc, 0),
        Some(Opcode::InA) => (Opcode::InA, 0),
        Some(Opcode::InB) => (Opcode::InB, 0),
        Some(Opcode::Out) => (Opcode::Out, im),
        Some(Opcode::OutB) => (Opcode::OutB, 0),
        _ => panic!("Error: Not implemented opcode: {:04b}", op),
    };
}

pub fn reset(register: &mut Register, port: &mut Port) {
    register.set_pc(0);
    register.set_c(0);
    register.set_a(0);
    register.set_b(0);

    port.set_input(0);
    port.set_output(0);
}

pub fn step(register: &mut Register, port: &mut Port, rom: Rom) {
    let data = fetch(register, rom);
    let (op, im) = decode(data);

    match op {
        Opcode::AddA => add_a(register, im),
        Opcode::AddB => add_b(register, im),
        Opcode::MovA => mov_a(register, im),
        Opcode::MovB => mov_b(register, im),
        Opcode::MovAB => mov_ab(register),
        Opcode::MovBA => mov_ba(register),
        Opcode::Jmp => jmp(register, im),
        Opcode::Jnc => jnc(register, im),
        Opcode::InA => in_a(register, port),
        Opcode::InB => in_b(register, port),
        Opcode::Out => out(register, port, im),
        Opcode::OutB => out_b(register, port),
    }
}

fn add(register: &mut Register, v: u8, im: u8) -> u8 {
    let n = v + im;
    if n > 0x0f {
        register.set_c(1);
    }
    return n & 0x0f;
}

fn add_a(register: &mut Register, im: u8) {
    let v = add(register, register.get_a(), im);
    register.set_a(v);
}

fn add_b(register: &mut Register, im: u8) {
    let v = add(register, register.get_b(), im);
    register.set_b(v);
}

fn mov_a(register: &mut Register, im: u8) {
    register.set_a(im);
    register.set_c(0);
}

fn mov_b(register: &mut Register, im: u8) {
    register.set_b(im);
    register.set_c(0);
}

fn mov_ab(register: &mut Register) {
    register.set_a(register.get_b());
    register.set_c(0);
}

fn mov_ba(register: &mut Register) {
    register.set_b(register.get_a());
    register.set_c(0);
}

fn jmp(register: &mut Register, im: u8) {
    register.set_pc(im);
    register.set_c(0);
}

fn jnc(register: &mut Register, im: u8) {
    if register.get_c() == 0 {
        register.set_pc(im);
    }
    register.set_c(0);
}

fn in_a(register: &mut Register, port: &mut Port) {
    register.set_a(port.get_input());
    register.set_c(0);
}

fn in_b(register: &mut Register, port: &mut Port) {
    register.set_b(port.get_input());
    register.set_c(0);
}

fn out(register: &mut Register, port: &mut Port, im: u8) {
    port.set_output(im);
    register.set_c(0);
}

fn out_b(register: &mut Register, port: &mut Port) {
    port.set_output(register.get_b());
    register.set_c(0);
}

#[cfg(test)]
mod tests {
    use crate::cpu;
    use crate::port::*;
    use crate::register::*;
    use crate::rom::*;

    fn step(
        data: u8,
        pre_registers: [u8; 4],
        expect_registers: [u8; 4],
        pre_ports: [u8; 2],
        expect_ports: [u8; 2],
    ) {
        let mut register = Register::new();
        let mut port = Port::new();
        let rom = Rom::new(vec![data]);

        register.set_pc(pre_registers[0]);
        register.set_c(pre_registers[1]);
        register.set_a(pre_registers[2]);
        register.set_b(pre_registers[3]);

        port.set_input(pre_ports[0]);
        port.set_output(pre_ports[1]);

        cpu::step(&mut register, &mut port, rom);

        assert_eq!(register.get_pc(), expect_registers[0]);
        assert_eq!(register.get_c(), expect_registers[1]);
        assert_eq!(register.get_a(), expect_registers[2]);
        assert_eq!(register.get_b(), expect_registers[3]);

        assert_eq!(port.get_input(), expect_ports[0]);
        assert_eq!(port.get_output(), expect_ports[1]);
    }

    #[test]
    fn test_add_a() {
        // ADD A, 0001
        step(
            0b00000001,
            [0b0000, 0b0000, 0b0001, 0b0000],
            [0b0001, 0b0000, 0b0010, 0b0000],
            [0b0000, 0b0000],
            [0b0000, 0b0000],
        );

        // ADD A, 0010 (carry)
        step(
            0b00000010,
            [0b0000, 0b0000, 0b1111, 0b0000],
            [0b0001, 0b0001, 0b0001, 0b0000],
            [0b0000, 0b0000],
            [0b0000, 0b0000],
        );
    }

    #[test]
    fn test_add_b() {
        // ADD B, 0010
        step(
            0b01010010,
            [0b0000, 0b0000, 0b0000, 0b0001],
            [0b0001, 0b0000, 0b0000, 0b0011],
            [0b0000, 0b0000],
            [0b0000, 0b0000],
        );

        // ADD B, 1110 (carry)
        step(
            0b01011110,
            [0b0000, 0b0000, 0b0000, 0b0011],
            [0b0001, 0b0001, 0b0000, 0b0001],
            [0b0000, 0b0000],
            [0b0000, 0b0000],
        );
    }

    #[test]
    fn test_mov_a() {
        // MOV A, 1111
        step(
            0b00111111,
            [0b0000, 0b0001, 0b0010, 0b0000],
            [0b0001, 0b0000, 0b1111, 0b0000],
            [0b0000, 0b0000],
            [0b0000, 0b0000],
        );
    }

    #[test]
    fn test_mov_b() {
        // MOB B, 0110
        step(
            0b01110110,
            [0b0000, 0b0001, 0b0000, 0b0111],
            [0b0001, 0b0000, 0b0000, 0b0110],
            [0b0000, 0b0000],
            [0b0000, 0b0000],
        );
    }

    #[test]
    fn test_mov_ab() {
        // MOB A, B
        step(
            0b00010000,
            [0b0000, 0b0001, 0b0000, 0b0010],
            [0b0001, 0b0000, 0b0010, 0b0010],
            [0b0000, 0b0000],
            [0b0000, 0b0000],
        );
    }

    #[test]
    fn test_mov_ba() {
        // MOB B, A
        step(
            0b01000000,
            [0b0000, 0b0001, 0b1111, 0b0000],
            [0b0001, 0b0000, 0b1111, 0b1111],
            [0b0000, 0b0000],
            [0b0000, 0b0000],
        );
    }

    #[test]
    fn test_jmp() {
        // JMP 0000
        step(
            0b11110000,
            [0b0000, 0b0001, 0b0000, 0b0000],
            [0b0000, 0b0000, 0b0000, 0b0000],
            [0b0000, 0b0000],
            [0b0000, 0b0000],
        );
    }

    #[test]
    fn test_jnc() {
        // JNC 0000 (no-carry)
        step(
            0b11100000,
            [0b0000, 0b0000, 0b0000, 0b0000],
            [0b0000, 0b0000, 0b0000, 0b0000],
            [0b0000, 0b0000],
            [0b0000, 0b0000],
        );

        // JNC 0000 (carry)
        step(
            0b11100000,
            [0b0000, 0b0001, 0b0000, 0b0000],
            [0b0001, 0b0000, 0b0000, 0b0000],
            [0b0000, 0b0000],
            [0b0000, 0b0000],
        );
    }

    #[test]
    fn test_in_a() {
        // IN A
        step(
            0b00100000,
            [0b0000, 0b0001, 0b1111, 0b0000],
            [0b0001, 0b0000, 0b0011, 0b0000],
            [0b0011, 0b0000],
            [0b0011, 0b0000],
        );
    }

    #[test]
    fn test_in_b() {
        // IN B
        step(
            0b01100000,
            [0b0000, 0b0001, 0b0000, 0b0111],
            [0b0001, 0b0000, 0b0000, 0b0100],
            [0b0100, 0b0000],
            [0b0100, 0b0000],
        );
    }

    #[test]
    fn test_out() {
        // OUT 1111
        step(
            0b10111111,
            [0b0000, 0b0001, 0b0000, 0b0000],
            [0b0001, 0b0000, 0b0000, 0b0000],
            [0b0000, 0b0000],
            [0b0000, 0b1111],
        );
    }

    #[test]
    fn test_out_b() {
        // OUT B
        step(
            0b10010000,
            [0b0000, 0b0001, 0b0000, 0b0010],
            [0b0001, 0b0000, 0b0000, 0b0010],
            [0b0000, 0b1111],
            [0b0000, 0b0010],
        );
    }

    #[test]
    fn test_reset() {
        let mut register = Register::new();
        let mut port = Port::new();

        register.set_pc(0b1111);
        register.set_c(0b0001);
        register.set_a(0b1111);
        register.set_b(0b1111);

        port.set_input(0b1111);
        port.set_output(0b1111);

        cpu::reset(&mut register, &mut port);

        assert_eq!(register.get_pc(), 0b0000);
        assert_eq!(register.get_c(), 0b0000);
        assert_eq!(register.get_a(), 0b0000);
        assert_eq!(register.get_b(), 0b0000);

        assert_eq!(port.get_input(), 0b0000);
        assert_eq!(port.get_output(), 0b0000);
    }
}
