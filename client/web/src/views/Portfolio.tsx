import React, { useState, useEffect } from 'react';
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import Table from "../components/Table";
import { useSelector, useDispatch } from 'react-redux'
import {
  getPrice,
  getPrices,
  fetchPricesThunk,
} from '../reducers/prices';
import {
  getPortfolios,
  fetchPortfolioThunk,
} from '../reducers/portfolio';


export function computePortfolio(portfolio, prices) {
  let aggr = {};

  for (let holding of portfolio.holdings) {
    let symbol = holding.symbol;
    let wallet = holding.locked ?
     `${holding.wallet} (${holding.locked})` :
     holding.wallet;
    if (!aggr.hasOwnProperty(symbol)) {
      aggr[symbol] = [{
        quantity: holding.quantity,
        wallet: wallet,
      }];
    } else {
      aggr[symbol].push({
        quantity: holding.quantity,
        wallet: wallet,
      });
    }
  }

  let results = [];

  for (let [key, value] of Object.entries(aggr)) {
    let sum = value.reduce((a, b) => a + b.quantity, 0);
    let price = getPrice(prices, key);

    let subRows = value.map((v) => {
      return {
        symbol: "",
        quantity: v.quantity,
        wallet: v.wallet,
        value: price * v.quantity,
      };
    }).sort((a, b) => b.quantity - a.quantity);

    let wallet = new Set();
    for (let row of subRows) {
      if (row.wallet === null) {
        continue;
      }
      if (row.value > price * sum * 0.1) {
        wallet.add(row.wallet);
      }
    }

    if (subRows.length < 2) {
      subRows = undefined;
    }

    results.push({
      symbol: key,
      quantity: sum,
      value: price * sum,
      wallet: Array.from(wallet).join(", "),
      subRows,
    });
  }

  results.sort((a, b) => b.value - a.value);
  return results;
}

function computeTable(portfolio = [], prices = []) {
  let cf = new Intl.NumberFormat(undefined, { style: 'currency', currency: 'USD' });
  let nf = new Intl.NumberFormat(undefined);

  let total = 0;
  let results = computePortfolio(portfolio, prices);

  results.forEach((entry) => {
    total += entry.value;
    entry.quantity = nf.format(entry.quantity);
    entry.value = cf.format(entry.value)
    if (entry.subRows) {
      entry.subRows.forEach((row) => {
        row.quantity = nf.format(row.quantity);
        row.value = cf.format(row.value);
      });
    }
  });
  return [results, cf.format(total)];
}

export default function Portfolio() {
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
      {
        Header: 'Wallet',
        accessor: "wallet",
      },
    ],
    []
  );

  const prices = useSelector(getPrices);
  const portfolios = useSelector(getPortfolios);

  const dispatch = useDispatch();
  useEffect(() => {
    dispatch(fetchPortfolioThunk())
  }, [dispatch])

  function getPanel(pf, prices) {
    let [data, total] = computeTable(pf, prices);
    return (
      <TabPanel key={`portfolio-tab-panel-${pf.id}`}>
        <span>Total: {total}</span>
        <Table
          columns={columns}
          data={data}
        />
      </TabPanel>
    );
  }

  return (
    <Tabs>
      <TabList>
        {portfolios.map(pf => (
          <Tab key={`portfolio-tab-${pf.id}`}>{pf.name}</Tab>
        ))}
      </TabList>

      {portfolios.map(pf => getPanel(pf, prices))}
    </Tabs>
  );
}
