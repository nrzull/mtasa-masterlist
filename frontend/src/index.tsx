import "normalize.css";
import "@/index.css";
import React from "react";
import ReactDOM from "react-dom";

import { Header } from "@/components/header";
import { Body } from "@/components/body";

function App() {
  return (
    <div>
      <Header />
      <Body />
    </div>
  );
}

ReactDOM.render(<App />, document.getElementById("root"));
