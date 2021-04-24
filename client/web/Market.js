import React, { useState, useEffect } from 'react';
import Table from "./Table";
import styled from 'styled-components';


async function getData(refresh = false) {
  let url = "http://127.0.0.1:8080/oracle/prices";
  if (refresh) {
    url += "?refresh=true";
  }

  let resp = await fetch(url, {});
  let {price, last_updated} = await resp.json();

  let cf = new Intl.NumberFormat(undefined, { style: 'currency', currency: 'USD' });
  let dtf = new Intl.DateTimeFormat(undefined, {dateStyle: "long", timeStyle: "long" });

  let values = price.map((entry) => {
    return {
      symbol: entry.pair[0],
      price: cf.format(entry.value),
    };
  });
  let date = new Date(last_updated);
  return [values, dtf.format(date)];
}

const Styles = styled.div`
  padding: 1rem;

  table {
    border-spacing: 0;
    border: 1px solid white;

    tr {
      :last-child {
        td {
          border-bottom: 0;
        }
      }
    }

    th,
    td {
      margin: 0;
      padding: 0.5rem;
      border-bottom: 1px solid white;
      border-right: 1px solid white;

      :last-child {
        border-right: 0;
      }
    }
  }
`;


export default function Market() {
  const columns = React.useMemo(
    () => [
      {
        // Build our expander column
        id: 'expander', // Make sure it has an ID
        Header: ({ getToggleAllRowsExpandedProps, isAllRowsExpanded }) => (
          <span {...getToggleAllRowsExpandedProps()}>
          </span>
        ),
        Cell: ({ row }) =>
          // Use the row.canExpand and row.getToggleRowExpandedProps prop getter
          // to build the toggle for expanding a row
          row.canExpand ? (
            <span
              {...row.getToggleRowExpandedProps({
                style: {
                  // We can even use the row.depth property
                  // and paddingLeft to indicate the depth
                  // of the row
                  paddingLeft: `${row.depth * 2}rem`,
                },
              })}
            >
              {row.isExpanded ? '▼' : '▶'}
            </span>
          ) : null,
      },
      {
        Header: 'Symbol',
        accessor: "symbol",
      },
      {
        Header: 'Exchange',
        accessor: "exchange",
      },
      {
        Header: 'Price',
        accessor: "price",
      },
    ],
    []
  );

  const [data, setData] = useState([]);
  const [lastUpdate, setLastUpdate] = useState("");

  useEffect(() => {
    loadData(false);
  }, []);

  function loadData(refresh = false) {
    getData(refresh).then(([newData, newLastUpdate]) => {
      setData(newData);
      setLastUpdate(newLastUpdate);
    });
  }

  return (
    <Styles>
      <p>
        Last update: {lastUpdate}
        <button onClick={() => loadData(true)}>
          Refresh
        </button>
      </p>
      <Table
        columns={columns}
        data={data}
      />
    </Styles>
  );
}
