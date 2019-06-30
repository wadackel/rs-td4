use num_derive::FromPrimitive;

// Operation Code
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum Opcode {
    AddA = 0b0000,  // ADD A, Im
    AddB = 0b0101,  // ADD B, Im
    MovA = 0b0011,  // MOV A, Im
    MovB = 0b0111,  // MOV B, Im
    MovAB = 0b0001, // MOV A, B
    MovBA = 0b0100, // MOV B, A
    Jmp = 0b1111,   // JMP Im
    Jnc = 0b1110,   // JNC Im
    InA = 0b0010,   // IN A
    InB = 0b0110,   // IN B
    Out = 0b1011,   // OUT Im
    OutB = 0b1001,  // OUT B
}
