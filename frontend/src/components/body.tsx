import "./body.css";
import React, { useState, ComponentProps } from "react";
import { WindowScroller } from "react-virtualized/dist/es/WindowScroller";
import { Table, Column } from "react-virtualized/dist/es/Table";
import { AutoSizer } from "react-virtualized/dist/es/AutoSizer";

import { TServerList } from "@/types";
import PadlockIcon from "@/assets/padlock.svg";
import PlayIcon from "@/assets/play.svg";
import SearchIcon from "@/assets/search.svg";
import ArrowDown from "@/assets/arrow-down.svg";

interface TProps extends ComponentProps<"div"> {
  list: TServerList;
}

type TColumnName = "name" | "online";
type TSort = "name-asc" | "name-desc" | "online-asc" | "online-desc";

const columns = 14;

function Body(p: TProps) {
  const [filter, setFilter] = useState("");
  const [sort, setSort] = useState<TSort>("online-desc");
  const [filterRegex, setFilterRegex] = useState(new RegExp(""));

  const onFilter = ev => {
    setFilter(ev.currentTarget.value);
    setFilterRegex(new RegExp(ev.currentTarget.value, "i"));
  };

  const onSort = ev => {
    const sortType = ev.currentTarget.getAttribute("data-sort");
    let result: TSort;

    switch (sortType) {
      case "online":
        {
          result = sort === "online-asc" ? "online-desc" : "online-asc";
        }
        break;

      case "name":
        {
          result = sort === "name-asc" ? "name-desc" : "name-asc";
        }
        break;
    }

    setSort(result);
  };

  const doFilter = servers => {
    if (!filter) return servers;
    return servers.filter(server => server.name.match(filterRegex));
  };

  const doSort = servers => {
    if (!sort) return servers;

    return servers.sort((s1, s2) => {
      switch (sort) {
        case "name-asc":
          return s1.name[0] >= s2.name[0] ? 1 : -1;

        case "name-desc":
          return s1.name[0] >= s2.name[0] ? -1 : 1;

        case "online-asc":
          return s1.players >= s2.players ? 1 : -1;

        case "online-desc":
          return s1.players >= s2.players ? -1 : 1;
      }
    });
  };

  const isAccent = (value: TColumnName) => {
    switch (value) {
      case "name":
        return sort === "name-asc" || sort === "name-desc";

      case "online":
        return sort === "online-asc" || sort === "online-desc";
    }
  };

  const list = doSort(doFilter(p.list));

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

        {!!list.length && (
          <div className="body__list">
            <WindowScroller>
              {({ height, isScrolling, onChildScroll, scrollTop }) => (
                <AutoSizer disableHeight>
                  {({ width }) => (
                    <Table
                      height={height}
                      isScrolling={isScrolling}
                      onScroll={onChildScroll}
                      scrollTop={scrollTop}
                      width={width}
                      rowHeight={60}
                      rowCount={list.length}
                      autoHeight
                      headerHeight={40}
                      headerClassName="body__list-head"
                      rowClassName="body__list-row"
                      rowGetter={({ index }) => list[index]}
                    >
                      <Column
                        label=""
                        dataKey="name"
                        className="body__list-cell body__list-cell_name text-bold"
                        headerClassName="body__list-header"
                        width={width - (width / columns) * 4}
                        cellRenderer={data => <>{data.rowData.name}</>}
                        headerRenderer={({ dataKey }) => (
                          <span
                            data-sort={dataKey}
                            onClick={onSort}
                            className="body__list-header-sort-cell"
                          >
                            {dataKey}{" "}
                            <ArrowDown
                              className="body__list-header-sort-icon"
                              data-accent={isAccent(dataKey as TColumnName)}
                              data-up={sort === `${dataKey}-asc`}
                            />
                          </span>
                        )}
                      />

                      <Column
                        label="public"
                        dataKey="public"
                        headerClassName="body__list-header body__list-header_public"
                        className="body__list-cell body__list-cell_public"
                        width={width / columns}
                        cellRenderer={data =>
                          data.rowData.version.includes("n") ? (
                            <a>
                              <PlayIcon
                                data-accent="false"
                                className="body__server-icon body__server-icon_disabled"
                              />
                            </a>
                          ) : (
                            <a
                              href={`mtasa://${data.rowData.ip}:${
                                data.rowData.port
                              }`}
                            >
                              <PlayIcon
                                data-accent="true"
                                className="body__server-icon"
                              />
                            </a>
                          )
                        }
                      />

                      <Column
                        label="password"
                        dataKey="password"
                        className="body__list-cell"
                        headerClassName="body__list-header"
                        width={width / columns}
                        cellRenderer={data => (
                          <>
                            <PadlockIcon
                              data-accent={!!data.rowData.password}
                              className="body__server-icon"
                            />
                          </>
                        )}
                      />

                      <Column
                        label="online"
                        dataKey="online"
                        className="body__list-cell text-bold text-accent"
                        headerClassName="body__list-header"
                        width={width / columns}
                        cellRenderer={data => <>{data.rowData.players}</>}
                        headerRenderer={({ dataKey }) => (
                          <span
                            data-sort={dataKey}
                            onClick={onSort}
                            className="body__list-header-sort-cell"
                          >
                            {dataKey}{" "}
                            <ArrowDown
                              className="body__list-header-sort-icon"
                              data-accent={isAccent(dataKey as TColumnName)}
                              data-up={sort === `${dataKey}-asc`}
                            />
                          </span>
                        )}
                      />

                      <Column
                        label="version"
                        dataKey="version"
                        className="body__list-cell body__list-cell_version text-bold"
                        headerClassName="body__list-header body__list-header_version"
                        width={width / columns}
                        cellRenderer={data => (
                          <>{parseFloat(data.rowData.version)}</>
                        )}
                      />
                    </Table>
                  )}
                </AutoSizer>
              )}
            </WindowScroller>
          </div>
        )}
      </div>
    </div>
  );
}

export { Body };
