import React, { useState, useEffect } from 'react';
import Table from "./Table";
import styled from 'styled-components';

async function getPriceData() {
  let url = "http://127.0.0.1:8080/prices";

  let resp = await fetch(url, {});
  let {price, last_updated} = await resp.json();

  let values = price.map((entry) => {
    return {
      symbol: entry.pair[0],
      price: entry.value,
    };
  });
  let date = new Date(last_updated);
  return [values, date];
}

function getPrice(prices, symbol) {
  if (symbol == "usd") {
    return 1;
  }

  for (let price of prices) {
    if (price.symbol == symbol) {
      return price.price;
    }
  }
  return null;
}

async function getData() {
  let url = "http://127.0.0.1:8080/portfolio";
  let prices = (await getPriceData())[0];

  let resp = await fetch(url);
  let json = await resp.json();

  let cf = new Intl.NumberFormat(undefined, { style: 'currency', currency: 'USD' });
  let nf = new Intl.NumberFormat(undefined);

  let aggr = {};

  for (let entry of json) {
    let symbol = entry.symbol;
    if (!aggr.hasOwnProperty(symbol)) {
      aggr[symbol] = [entry.quantity];
    } else {
      aggr[symbol].push(entry.quantity);
    }
  }

  let results = [];
  let total = 0;

  for (let [key, value] of Object.entries(aggr)) {
    let sum = value.reduce((a, b) => a + b, 0);
    let price = getPrice(prices, key);
    total += sum * price;

    let subRows = value.map((v) => {
      return {
        symbol: "",
        quantity: nf.format(v),
        value: cf.format(price * v),
      };
    });
    subRows.sort((a, b) => {
      return b.quantity - a.quantity;
    });

    if (subRows.length < 2) {
      subRows = undefined;
    }

    results.push({
      symbol: key,
      quantity: nf.format(sum),
      value: price * sum,
      subRows,
    });
  }

  results.sort((a, b) => {
    return b.value - a.value;
  });
  results.forEach((entry) => {
    entry.value = cf.format(entry.value)
  });
  return [results, cf.format(total)];
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
        Header: 'Quantity',
        accessor: "quantity",
      },
      {
        Header: 'Value',
        accessor: "value",
      },
    ],
    []
  );

  const [data, setData] = useState([]);
  const [total, setTotal] = useState("");

  useEffect(() => {
    onRefresh();
  }, []);

  function onRefresh() {
    getData().then(([newData, newTotal]) => {
      setData(newData);
      setTotal(newTotal);
    });
  }

  return (
    <Styles>
      <span>Total: {total}</span>
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
