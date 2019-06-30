import React from "react";
import styled from "styled-components";

const Wrapper = styled.header`
  display: flex;
  justify-content: space-between;
  align-items: center;

  & h1 {
    margin: 0;
    font-size: 1rem;
  }
`;

const GitHubLink = styled.a`
  display: flex;

  & .nes-balloon {
    align-self: flex-start;
    color: #333;
    padding: 0.2rem 0.5rem;
    font-size: 0.8rem;
  }

  &:hover {
    text-decoration: none;
  }
`;

export const Header: React.FC = () => (
  <Wrapper>
    <h1>WebAssembly TD4 Emulator with Rust</h1>
    <GitHubLink href="https://github.com/tsuyoshiwada/rs-td4">
      <p className="nes-balloon from-right">
        Fork me
        <br />
        on GitHub
      </p>
      <i className="nes-octocat" />
    </GitHubLink>
  </Wrapper>
);
