import "normalize.css";
import "@/index.css";
import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import { TServerList } from "@/types";

import { Header } from "@/components/header";
import { Body } from "@/components/body";

function App() {
  const [list, setList] = useState<TServerList>([]);

  const fetchList = () => {
    fetch("/api/list")
      .then(res => res.json())
      .then((servers: TServerList) => {
        setList(
          servers
            .filter(s => !!s.name)
            .sort((s1, s2) => (s1.players >= s2.players ? -1 : 1))
        );
        setTimeout(fetchList, 1000 * 30);
      });
  };

  useEffect(fetchList, []);

  return (
    <div>
      <Header />
      <Body list={list} />
    </div>
  );
}

ReactDOM.render(<App />, document.getElementById("root"));
