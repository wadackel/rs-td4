import React from "react";
import ReactDOM from "react-dom";
import { App } from "./components/App";
import { load } from "./module";

(async () => {
  try {
    await load();
    ReactDOM.render(<App />, document.getElementById("app"));
  } catch (e) {
    console.error(e);
  }
})();
