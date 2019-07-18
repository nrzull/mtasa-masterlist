import "./body.css";
import React, { useState, ComponentProps } from "react";
import { TServerList } from "@/types";
import SearchIcon from "@/assets/search.svg";
import { List } from "@/components/list";
import { Checkbox } from "@/components/checkbox";

interface TProps extends ComponentProps<"div"> {
  list: TServerList;
}

type TOptionNames = "locked" | "empty" | "full";

type TOptionStorage = {
  locked: boolean;
  empty: boolean;
  full: boolean;
};

const optionsStorageKey = "body-options";
const optionsStorageResult: string = localStorage.getItem(optionsStorageKey);
const optionsStorage: TOptionStorage = optionsStorageResult
  ? JSON.parse(optionsStorageResult)
  : { empty: true, full: true, locked: true };

function Body(p: TProps) {
  const [filter, setFilter] = useState("");
  const [filterRegex, setFilterRegex] = useState(new RegExp(""));
  const [locked, setLocked] = useState(optionsStorage.locked);
  const [empty, setEmpty] = useState(optionsStorage.empty);
  const [full, setFull] = useState(optionsStorage.full);

  const onFilter = ev => {
    setFilter(ev.currentTarget.value);
    setFilterRegex(new RegExp(ev.currentTarget.value, "i"));
  };

  const doFilter = servers => {
    if (!filter) return servers;
    return servers.filter(server => server.name.match(filterRegex));
  };

  const onOption = (name: TOptionNames) => ev => {
    switch (name) {
      case "locked": {
        optionsStorage.locked = ev.currentTarget.checked;
        localStorage.setItem(optionsStorageKey, JSON.stringify(optionsStorage));
        return setLocked(ev.currentTarget.checked);
      }

      case "empty": {
        optionsStorage.empty = ev.currentTarget.checked;
        localStorage.setItem(optionsStorageKey, JSON.stringify(optionsStorage));
        return setEmpty(ev.currentTarget.checked);
      }

      case "full": {
        optionsStorage.full = ev.currentTarget.checked;
        localStorage.setItem(optionsStorageKey, JSON.stringify(optionsStorage));
        return setFull(ev.currentTarget.checked);
      }
    }
  };

  const doOption = (servers: TServerList) => {
    if (!locked) servers = servers.filter(s => s.password != 1);
    if (!empty) servers = servers.filter(s => s.players != 0);
    if (!full) servers = servers.filter(s => s.players != s.maxplayers);

    return servers;
  };

  const getPlayersCount = list => {
    return list.reduce((acc, server) => {
      acc += server.players;
      return acc;
    }, 0);
  };

  const list = doOption(doFilter(p.list));

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
              <span className="text-accent">{getPlayersCount(list)}</span>
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

        <div className="body__options-container">
          <Checkbox onChange={onOption("locked")} checked={locked}>
            Locked
          </Checkbox>

          <Checkbox onChange={onOption("empty")} checked={empty}>
            Empty
          </Checkbox>

          <Checkbox onChange={onOption("full")} checked={full}>
            Full
          </Checkbox>
        </div>

        {!!list.length && <List list={list} />}
      </div>
    </div>
  );
}

export { Body };
