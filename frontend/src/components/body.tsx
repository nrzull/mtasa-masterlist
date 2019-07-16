import "./body.css";
import React, { useState, ComponentProps } from "react";
import { TServerList } from "@/types";
import SearchIcon from "@/assets/search.svg";
import { List } from "@/components/list";

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

  const list = doFilter(p.list);

  return (
    <div className="body">
      <div className="container">
        <div className="body__toolbox">
          <div className="body__stats">
            <span>
              servers: <span className="text-accent">{list.length}</span>
            </span>

            <span>
              online:{" "}
              <span className="text-accent">
                {list.reduce((acc, curr) => {
                  acc += curr.players;
                  return acc;
                }, 0)}
              </span>
            </span>
          </div>

          <div className="body__search-container">
            <input
              className="body__search"
              onChange={onFilter}
              type="text"
              value={filter}
              placeholder="search"
            />
            <SearchIcon className="body__search-icon" />
          </div>
        </div>

        {!!list.length && <List list={list} />}
      </div>
    </div>
  );
}

export { Body };
