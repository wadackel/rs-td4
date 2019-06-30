#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate num_derive;

mod cpu;
mod opcode;
mod port;
mod register;
mod rom;

use crate::port::*;
use crate::register::*;
use crate::rom::*;
use std::sync::Mutex;

lazy_static! {
    static ref ROM: [u8; 16] = [0; 16];
    static ref REGISTER: Mutex<[u8; 4]> = Mutex::new([0; 4]);
    static ref PORT: Mutex<[u8; 2]> = Mutex::new([0; 2]);
}

#[no_mangle]
pub fn get_rom_ptr() -> *const u8 {
    return ROM.as_ptr();
}

#[no_mangle]
pub fn get_register_ptr() -> *const u8 {
    return REGISTER.lock().unwrap().as_ptr();
}

#[no_mangle]
pub fn get_port_ptr() -> *const u8 {
    return PORT.lock().unwrap().as_ptr();
}

fn register2mem(register: Register) {
    let mut arr = REGISTER.lock().unwrap();
    arr[0] = register.get_pc();
    arr[1] = register.get_c();
    arr[2] = register.get_a();
    arr[3] = register.get_b();
}

fn mem2register(register: &mut Register) {
    let arr = REGISTER.lock().unwrap();
    register.set_pc(arr[0]);
    register.set_c(arr[1]);
    register.set_a(arr[2]);
    register.set_b(arr[3]);
}

fn port2mem(port: Port) {
    let mut arr = PORT.lock().unwrap();
    arr[0] = port.get_input();
    arr[1] = port.get_output();
}

fn mem2port(port: &mut Port) {
    let arr = PORT.lock().unwrap();
    port.set_input(arr[0]);
    port.set_output(arr[1]);
}

fn restore_state() -> (Register, Port) {
    let mut register = Register::new();
    mem2register(&mut register);

    let mut port = Port::new();
    mem2port(&mut port);

    return (register, port);
}

fn memory_sync(register: Register, port: Port) {
    register2mem(register);
    port2mem(port);
}

#[no_mangle]
pub fn step() {
    let rom_vec = ROM.to_vec();
    let rom = Rom::new(rom_vec);

    let (mut register, mut port) = restore_state();

    cpu::step(&mut register, &mut port, rom);

    memory_sync(register, port);
}

#[no_mangle]
pub fn reset() {
    let (mut register, mut port) = restore_state();

    cpu::reset(&mut register, &mut port);

    memory_sync(register, port);
}
