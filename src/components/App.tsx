import React, { useState, useEffect, useCallback } from "react";
import { GlobalStyle } from "../styles";
import { Header } from "./Header";
import { Footer } from "./Footer";
import styled from "styled-components";
import { BitList } from "./BitList";
import { Container } from "./Container";
import { Status } from "./Status";
import { Clock } from "./Clock";
import * as cpu from "../module";

const Wrapper = styled.div`
  display: grid;
  grid-template-rows: 120px 1fr 120px;
  grid-template-columns: 1fr;
  height: 100%;
`;

const Main = styled.main`
  display: grid;
  grid-template-columns: 0.5fr 0.5fr;
  grid-row-gap: 32px;
  grid-column-gap: 32px;
`;

const Registers = styled.div`
  & > *:not(:first-child) {
    margin-top: 32px;
  }
`;

const INITIAL_ROM = [
  0b10110011,
  0b10110110,
  0b10111100,
  0b10111000,
  0b10111000,
  0b10111100,
  0b10110110,
  0b10110011,
  0b10110001,
  0b11110000,
  0,
  0,
  0,
  0,
  0,
  0
];

// ramen timer
// const INITIAL_ROM = [
//   0b10110111,
//   0b00000001,
//   0b11100001,
//   0b00000001,
//   0b11100011,
//   0b10110110,
//   0b00000001,
//   0b11100110,
//   0b00000001,
//   0b11101000,
//   0b10110000,
//   0b10110100,
//   0b00000001,
//   0b11101010,
//   0b10111000,
//   0b11111111,
// ];

export type Props = {};

type CpuState = {
  rom: number[];
  register: number[];
  port: number[];
};

export const App: React.FC<Props> = () => {
  const [state, setState] = useState<CpuState>({
    rom: Array.from<number>({ length: 16 }).fill(0),
    register: Array.from<number>({ length: 4 }).fill(0),
    port: Array.from<number>({ length: 2 }).fill(0)
  });

  const [autoClock, setAutoClock] = useState(false);

  // utilities
  const sync = () => {
    setState({
      rom: cpu.getRom(),
      register: cpu.getRegister(),
      port: cpu.getPort()
    });
  };

  const step = () => {
    cpu.step();
    sync();
  };

  // handlers
  const handleStep = useCallback(() => {
    if (autoClock) {
      setAutoClock(false);
    }

    step();
  }, [autoClock]);

  const handleStart = useCallback(() => {
    setAutoClock(true);
    step();
  }, []);

  const handleReset = useCallback(() => {
    cpu.reset();
    sync();
    setAutoClock(false);
  }, []);

  const handleInput = useCallback((input: number) => {
    const [, output] = cpu.getPort();
    cpu.setPort([input, output]);
    sync();
  }, []);

  const handleRomChange = useCallback(
    (value: number, label: string | undefined) => {
      const pos = parseInt(label == null ? "1" : label, 10);
      const rom = [...cpu.getRom()];
      rom[pos - 1] = value;
      cpu.setRom(rom);
      sync();
    },
    [state]
  );

  // effects
  useEffect(() => {
    cpu.setRom(INITIAL_ROM);
    sync();
  }, []);

  useEffect(() => {
    const timerId = setInterval(() => {
      if (autoClock) {
        step();
      }
    }, 1000);

    return () => {
      clearTimeout(timerId);
    };
  }, [autoClock]);

  return (
    <Wrapper>
      <GlobalStyle />

      <Header />

      <Main>
        <Container title="INPUT">
          <BitList bit={4} value={state.port[0]} onChange={handleInput} />
        </Container>

        <Container title="OUTPUT">
          <BitList bit={4} disabled value={state.port[1]} />
        </Container>

        <Registers>
          <Container title="REGISTERS">
            <BitList disabled label="A" bit={4} value={state.register[2]} />
            <BitList disabled label="B" bit={4} value={state.register[3]} />
          </Container>

          <Container title="STATUS">
            <Status pc={state.register[0]} carry={state.register[1] === 1} />
          </Container>

          <Container title="CLOCK">
            <Clock
              onStep={handleStep}
              onStart={handleStart}
              onReset={handleReset}
            />
          </Container>
        </Registers>

        <Container title="ROM">
          {state.rom.map((v, i) => (
            <BitList
              key={i}
              label={`${i + 1}`}
              bit={8}
              value={v}
              onChange={handleRomChange}
            />
          ))}
        </Container>
      </Main>

      <Footer />
    </Wrapper>
  );
};
