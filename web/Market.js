import React, { useState, useEffect } from 'react';
import Table from "./Table";
import styled from 'styled-components';


async function getData() {
  let url = "http://127.0.0.1:8080/prices";

  let resp = await fetch(url);
  let json = await resp.json();

  return json.map((entry) => {
    return {
      symbol: entry.pair[0],
      price: entry.value
    };
  });
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

  useEffect(() => {
    onRefresh();
  }, []);

  function onRefresh() {
    getData().then(newData => {
      setData(newData);
    });
  }

  return (
    <Styles>
      <Table
        columns={columns}
        data={data}
      />
      <button onClick={() => onRefresh()}>
        Click me
      </button>
    </Styles>
  );
}
