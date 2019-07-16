import "./list.css";
import React, { useState, ComponentProps } from "react";
import { WindowScroller } from "react-virtualized/dist/es/WindowScroller";
import { Table, Column } from "react-virtualized/dist/es/Table";
import { AutoSizer } from "react-virtualized/dist/es/AutoSizer";

import PadlockIcon from "@/assets/padlock.svg";
import PlayIcon from "@/assets/play.svg";
import ArrowDown from "@/assets/arrow-down.svg";
import { TServerList } from "@/types";

type TColumnName = "name" | "online";
type TSort = "name-asc" | "name-desc" | "online-asc" | "online-desc";

interface TProps extends ComponentProps<"div"> {
  list: TServerList;
}

const columns = 14;

function List(p: TProps) {
  const [sort, setSort] = useState<TSort>("online-desc");

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

  const list = doSort(p.list);

  return (
    <div className="list">
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
                headerClassName="list__header"
                rowClassName="list__row"
                rowGetter={({ index }) => list[index]}
              >
                <Column
                  label=""
                  dataKey="name"
                  className="list__cell list__cell_name text-bold"
                  headerClassName="list__header_name"
                  width={width - (width / columns) * 4}
                  cellRenderer={data => <>{data.rowData.name}</>}
                  headerRenderer={({ dataKey }) => (
                    <span
                      data-sort={dataKey}
                      onClick={onSort}
                      className="list__header-sort-cell"
                    >
                      {dataKey}{" "}
                      <ArrowDown
                        className="list__header-sort-icon"
                        data-accent={isAccent(dataKey as TColumnName)}
                        data-up={sort === `${dataKey}-asc`}
                      />
                    </span>
                  )}
                />

                <Column
                  label="public"
                  dataKey="public"
                  headerClassName="list__header_public"
                  className="list__cell list__cell_public"
                  width={width / columns}
                  cellRenderer={data =>
                    data.rowData.version.includes("n") ? (
                      <a>
                        <PlayIcon
                          data-accent="false"
                          className="list__server-icon list__server-icon_disabled"
                        />
                      </a>
                    ) : (
                      <a
                        href={`mtasa://${data.rowData.ip}:${data.rowData.port}`}
                      >
                        <PlayIcon
                          data-accent="true"
                          className="list__server-icon"
                        />
                      </a>
                    )
                  }
                />

                <Column
                  label="password"
                  dataKey="password"
                  className="list__cell"
                  width={width / columns}
                  cellRenderer={data => (
                    <>
                      <PadlockIcon
                        data-accent={!!data.rowData.password}
                        className="list__server-icon"
                      />
                    </>
                  )}
                />

                <Column
                  label="online"
                  dataKey="online"
                  className="list__cell text-bold text-accent"
                  width={width / columns}
                  cellRenderer={data => <>{data.rowData.players}</>}
                  headerRenderer={({ dataKey }) => (
                    <span
                      data-sort={dataKey}
                      onClick={onSort}
                      className="list__header-sort-cell"
                    >
                      {dataKey}{" "}
                      <ArrowDown
                        className="list__header-sort-icon"
                        data-accent={isAccent(dataKey as TColumnName)}
                        data-up={sort === `${dataKey}-asc`}
                      />
                    </span>
                  )}
                />

                <Column
                  label="version"
                  dataKey="version"
                  className="list__cell list__cell_version text-bold"
                  headerClassName="list__header_version"
                  width={width / columns}
                  cellRenderer={data => <>{parseFloat(data.rowData.version)}</>}
                />
              </Table>
            )}
          </AutoSizer>
        )}
      </WindowScroller>
    </div>
  );
}

export { List };
