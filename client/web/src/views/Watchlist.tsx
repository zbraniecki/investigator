import React, { useState, useEffect } from 'react';
import Table from "../components/Table";
import styled from 'styled-components';
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';

async function getPriceData() {
  let url = "http://127.0.0.1:8080/oracle/prices";

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


async function getWatchlistData() {
  let url = "http://127.0.0.1:8081/account/watchlist";
  let prices = (await getPriceData())[0];

  let resp = await fetch(url, {});
  let watchlists = await resp.json();

  let result = watchlists.map((entry) => {
    return {
      name: entry.name,
      coins: entry.coins.map(coin => {
        return {
          'symbol': coin,
          'price': getPrice(prices, coin),
        };
      }),
    };
  });
  return result;
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


export default function Watchlist() {
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
        Header: 'Price',
        accessor: "price",
      },
    ],
    []
  );

  const [watchlistData, setWatchlistData] = useState({
    name: "",
    coins: []
  });

  useEffect(() => {
    loadData();
  }, []);

  function loadData() {
    getWatchlistData().then((newData) => {
      let watchlist = newData[0];
      setWatchlistData(watchlist);
    });
  }

  return (
    <Styles>
    <Tabs>
      <TabList>
        <Tab>{watchlistData.name}</Tab>
      </TabList>

      <TabPanel>
        <Table
          columns={columns}
          data={watchlistData.coins}
        />
      </TabPanel>
    </Tabs>
    </Styles>
  );
}
