import "./body.css";
import React, { useState, ComponentProps } from "react";

import PadlockIcon from "@/assets/padlock.svg";
import PlayIcon from "@/assets/play.svg";
import { TServerList } from "@/types";

interface TProps extends ComponentProps<"div"> {
  list: TServerList;
}

function Body(p: TProps) {
  const [filter, setFilter] = useState("");
  const [filterRegex, setFilterRegex] = useState(new RegExp(""));

  const onFilter = ev => {
    setFilter(ev.currentTarget.value);
    setFilterRegex(new RegExp(ev.currentTarget.value, "i"));
  };

  const doFilter = servers => {
    if (!filter) return servers;
    return servers.filter(server => server.name.match(filterRegex));
  };

  return (
    <div className="body">
      <div className="container">
        <div className="body__toolbox">
          <div className="body__stats">
            <span>
              servers: <span className="accent">{p.list.length}</span>
            </span>

            <span>
              online:{" "}
              <span className="accent">
                {p.list.reduce((acc, curr) => {
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

        <table className="body__list">
          <thead>
            <tr>
              <td />
              <td className="body__list-column-head">public</td>
              <td className="body__list-column-head">password</td>
              <td className="body__list-column-head">online</td>
              <td className="body__list-column-head">version</td>
            </tr>
          </thead>
          <tbody>
            {doFilter(p.list).map((server, i) => (
              <tr key={i} className="body__server">
                <td className="bold body__server-name">{server.name}</td>
                <td className="body__server-ip">
                  {!!server.version.includes("n") ? (
                    <a>
                      <PlayIcon className="body__server-icon disabled" />
                    </a>
                  ) : (
                    <a href={`mtasa://${server.ip}:${server.port}`}>
                      <PlayIcon
                        data-accent="true"
                        className="body__server-icon"
                      />
                    </a>
                  )}
                </td>
                <td>
                  <PadlockIcon
                    data-accent={!!server.password}
                    className="body__server-icon"
                  />
                </td>
                <td className="accent bold ">{server.players}</td>
                <td className="bold">{parseFloat(server.version)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export { Body };
