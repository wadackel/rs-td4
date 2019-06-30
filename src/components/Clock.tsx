import React from "react";
import styled from "styled-components";

const Wrapper = styled.div`
  display: flex;
  align-items: center;

  & > *:not(:first-child) {
    margin-left: 6px;
  }
`;

export type Props = {
  onStep(): void;
  onStart(): void;
  onReset(): void;
};

export const Clock: React.FC<Props> = ({ onStep, onStart, onReset }) => {
  return (
    <Wrapper>
      <span>
        <button type="button" className="nes-btn" onClick={onStep}>
          +
        </button>
      </span>
      <span>
        <button type="button" className="nes-btn" onClick={onStart}>
          START
        </button>
      </span>
      <span>
        <button type="button" className="nes-btn" onClick={onReset}>
          RESET
        </button>
      </span>
    </Wrapper>
  );
};
