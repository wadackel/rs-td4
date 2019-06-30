import React from "react";
import styled from "styled-components";

const Wrapper = styled.div`
  & > div {
    display: flex;
    align-items: center;
  }

  & > div:not(:first-child) {
    margin-top: 6px;
  }

  & > div > span:first-child {
    flex-basis: 3rem;
    margin: 0;
    padding-right: 6px;
    text-align: right;
  }
`;

export type Props = {
  pc: number;
  carry: boolean;
};

export const Status: React.FC<Props> = ({ pc, carry }) => {
  return (
    <Wrapper>
      <div>
        <span>PC</span>
        <span>{pc}</span>
      </div>

      <div>
        <span>C</span>
        <span>
          <input
            type="checkbox"
            className="nes-checkbox"
            disabled
            checked={carry}
          />
          <span></span>
        </span>
      </div>
    </Wrapper>
  );
};
