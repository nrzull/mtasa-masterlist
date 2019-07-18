import "normalize.css";
import "@/index.css";
import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import { TServerList } from "@/types";

import { Header } from "@/components/header";
import { Body } from "@/components/body";
import { Loading } from "@/components/loading";

function App() {
  const [list, setList] = useState<TServerList>([]);

  const fetchList = () => {
    fetch("/api/list")
      .then(res => res.json())
      .then((servers: TServerList) => {
        setList(servers);
        setTimeout(fetchList, 1000 * 30);
      });
  };

  useEffect(fetchList, []);

  if (list.length === 0) return <Loading />;

  return (
    <>
      <Header />
      <Body list={list} />
    </>
  );
}

ReactDOM.render(<App />, document.getElementById("root"));
