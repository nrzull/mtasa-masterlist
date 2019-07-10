import "./body.css";
import React, { useEffect, useState } from "react";

type TServer = {
  ip: string;
  maxplayers: number;
  name: string;
  password: number;
  players: number;
  port: number;
  version: string;
};

type TList = TServer[];

function Body() {
  const [list, setList] = useState<TList>([]);
  const [filter, setFilter] = useState("");
  const [filterRegex, setFilterRegex] = useState(new RegExp(""));

  const fetchData = () => {
    fetch("http://localhost:8081/api/list")
      .then(res => res.json())
      .then((data: TList) => {
        setList(data);
        setTimeout(fetchData, 5000);
      });
  };

  useEffect(fetchData, []);

  const onFilter = ev => {
    setFilter(ev.currentTarget.value);
    setFilterRegex(new RegExp(ev.currentTarget.value, "i"));
  };

  return (
    <div className="body">
      <div className="body__container">
        <div className="body__toolbox">
          <div className="body__stats">
            <span>
              servers: <span className="accent">{list.length}</span>
            </span>

            <span>
              online:{" "}
              <span className="accent">
                {list.reduce((acc, curr) => {
                  acc += curr.players;
                  return acc;
                }, 0)}
              </span>
            </span>
          </div>

          <div>
            <input
              className="body__search"
              onChange={onFilter}
              type="text"
              value={filter}
              placeholder="search"
            />
          </div>
        </div>

        <div className="body__list">
          {list
            .sort((s1, s2) => (s1.players >= s2.players ? -1 : 1))
            .filter(server => {
              if (!filter) return server;
              if (server.name.match(filterRegex)) return server;
            })
            .map((server, i) => (
              <div key={i} className="body__server">
                <div className="body__server-name">{server.name}</div>

                <div className="body__server-labels">
                  <div className="accent">{server.players}</div>
                  <div>{server.version}</div>
                </div>
              </div>
            ))}
        </div>
      </div>
    </div>
  );
}

export { Body };
