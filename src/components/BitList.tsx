import React, { useCallback } from "react";
import styled from "styled-components";

const Wrapper = styled.div`
  position: relative;
  display: flex;
  align-items: center;
  margin: 6px auto;
`;

const Label = styled.span`
  flex-basis: calc(2rem + 6px);
  padding-right: 6px;
  text-align: right;
`;

const Ol = styled.ol`
  display: flex;
  align-items: center;
  margin: 0;
  padding: 0;
  list-style: none;

  & > li:not(:first-child) {
    margin-left: 6px;
  }
`;

const dec2bits = (num: number, bit: number) =>
  num
    .toString(2)
    .padStart(bit, "0")
    .split("")
    .map(v => parseInt(v, 10));

const bits2dec = (bits: number[]) => parseInt(bits.join(""), 2);

export type Props = {
  bit: 4 | 8;
  value: number;
  label?: string;
  disabled?: boolean;
  onChange?(value: number, label: string | undefined): void;
};

export const BitList: React.FC<Props> = ({
  bit,
  label,
  value,
  disabled,
  onChange
}) => {
  const bits = dec2bits(value, bit);

  const handleClick = useCallback(
    (e: React.MouseEvent<HTMLButtonElement>) => {
      if (onChange == null) {
        return;
      }

      const index = parseInt(e.currentTarget.value);
      const newBits = [...bits];
      newBits[index] = newBits[index] === 0 ? 1 : 0;

      onChange(bits2dec(newBits), label);
    },
    [value, label]
  );

  return (
    <Wrapper>
      {label != null ? <Label>{label}</Label> : null}
      <Ol>
        {bits.map((bit, i) => (
          <li key={i}>
            <button
              type="button"
              value={i}
              className={`nes-btn ${disabled ? "is-disabled" : ""}`}
              onClick={handleClick}
            >
              {bit}
            </button>
          </li>
        ))}
      </Ol>
    </Wrapper>
  );
};
