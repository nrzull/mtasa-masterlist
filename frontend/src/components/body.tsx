import "./body.css";
import React, { useEffect, useState } from "react";

type TServer = {
  ip: string;
  maxplayers: number;
  name: string;
  password: number;
  players: number;
  port: number;
  keep: number;
  version: string;
};

type TServerList = TServer[];

function Body() {
  const [list, setList] = useState<TServerList>([]);
  const [filter, setFilter] = useState("");
  const [filterRegex, setFilterRegex] = useState(new RegExp(""));

  const fetchData = () => {
    fetch("http://localhost:8081/api/list")
      .then(res => res.json())
      .then((servers: TServerList) => {
        setList(servers.filter(s => !!s.name));
        setTimeout(fetchData, 1000 * 30);
      });
  };

  useEffect(fetchData, []);

  const onFilter = ev => {
    setFilter(ev.currentTarget.value);
    setFilterRegex(new RegExp(ev.currentTarget.value, "i"));
  };

  return (
    <div className="body">
      <div className="container">
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

        {!!list.length && (
          <table className="body__list">
            <thead>
              <tr>
                <td />
                <td />
                <td className="body__list-column-head">online</td>
                <td className="body__list-column-head">version</td>
              </tr>
            </thead>
            <tbody>
              {list
                .sort((s1, s2) => (s1.players >= s2.players ? -1 : 1))
                .filter(server => {
                  if (!filter) return server;
                  if (server.name.match(filterRegex)) return server;
                })
                .map((server, i) => (
                  <tr key={i} className="body__server">
                    <td className="bold body__server-name">{server.name}</td>
                    <td className="body__server-ip">
                      {!!server.keep ? (
                        <a href={`mtasa://${server.ip}:${server.port}`}>play</a>
                      ) : (
                        <a className="disabled">play</a>
                      )}
                    </td>
                    <td className="accent bold ">{server.players}</td>
                    <td className="bold">{parseFloat(server.version)}</td>
                  </tr>
                ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
}

export { Body };
