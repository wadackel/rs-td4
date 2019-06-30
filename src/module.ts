type Exports = {
  memory: WebAssembly.Memory;
  get_rom_ptr(): number;
  get_register_ptr(): number;
  get_port_ptr(): number;
  reset(): void;
  step(): void;
};

type SharedMemory = {
  rom: Uint8Array;
  register: Uint8Array;
  port: Uint8Array;
};

let exports: Exports | null = null;
let shared: SharedMemory | null = null;

const guard = () => new Error("WebAssembly instance does not exist");

export const load = async () => {
  const { instance } = await WebAssembly.instantiateStreaming(
    fetch("./td4_bg.wasm"),
    {
      env: {}
    }
  );

  exports = instance.exports as Exports;

  const pointers = {
    rom: exports.get_rom_ptr(),
    register: exports.get_register_ptr(),
    port: exports.get_port_ptr()
  };

  shared = {
    rom: new Uint8Array(exports.memory.buffer, pointers.rom, 16),
    register: new Uint8Array(exports.memory.buffer, pointers.register, 4),
    port: new Uint8Array(exports.memory.buffer, pointers.port, 2)
  };
};

export const reset = () => {
  if (exports == null) {
    throw guard();
  }

  exports.reset();
};

export const step = () => {
  if (exports == null) {
    throw guard();
  }

  exports.step();
};

export const getRom = () => {
  if (shared == null) {
    throw guard();
  }

  return Array.from(shared.rom);
};

export const setRom = (data: number[]) => {
  if (shared == null) {
    throw guard();
  }

  for (let i = 0; i < data.length; i++) {
    shared.rom[i] = data[i];
  }
};

export const getRegister = () => {
  if (shared == null) {
    throw guard();
  }

  return Array.from(shared.register);
};

export const getPort = () => {
  if (shared == null) {
    throw guard();
  }

  return Array.from(shared.port);
};

export const setPort = (data: number[]) => {
  if (shared == null) {
    throw guard();
  }

  for (let i = 0; i < data.length; i++) {
    shared.port[i] = data[i];
  }
};
