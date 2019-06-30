import React from "react";
import styled from "styled-components";

const Wrapper = styled.footer`
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 12px;

  & * {
    margin: 0;
  }
`;

export const Footer: React.FC = () => (
  <Wrapper>
    <p>
      <a href="https://github.com/tsuyoshiwada/rs-td4">
        <i className="nes-icon github is-small" /> tsuyoshiwada/rs-td4
      </a>
    </p>
    <p>
      Made with <i className="nes-icon is-small heart" />{" "}
      <a href="https://github.com/tsuyoshiwada">@tsuyoshiwada</a>.
    </p>
  </Wrapper>
);
