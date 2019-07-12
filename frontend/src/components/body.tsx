import "./body.css";
import React, { useState, ComponentProps } from "react";
import { WindowScroller } from "react-virtualized/dist/es/WindowScroller";
import { Table, Column } from "react-virtualized/dist/es/Table";
import { AutoSizer } from "react-virtualized/dist/es/AutoSizer";

import { TServerList } from "@/types";
import PadlockIcon from "@/assets/padlock.svg";
import PlayIcon from "@/assets/play.svg";
import SearchIcon from "@/assets/search.svg";

interface TProps extends ComponentProps<"div"> {
  list: TServerList;
}

const columns = 16;

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
                              <PlayIcon className="body__server-icon body__server-icon_disabled" />
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
