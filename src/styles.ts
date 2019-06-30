import { createGlobalStyle } from "styled-components";

export const GlobalStyle = createGlobalStyle`
  html,
  body,
  #app {
    width: 100%;
    height: 100%;
  }

  body {
    padding: 0 30px;
  }

  /* overwrite NES.css styles */
  .nes-btn {
    padding: 0 4px;
  }
`;
