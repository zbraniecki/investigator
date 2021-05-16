import React, { useState, useEffect } from 'react';
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import Table from "../components/Table";
import { useSelector, useDispatch } from 'react-redux'
import {
  getPrices,
  fetchPricesThunk,
} from '../reducers/prices';
import {
  getPortfolios,
  fetchPortfolioThunk,
} from '../reducers/portfolio';
import {
  getWallets,
  getWallet,
  fetchWalletsThunk,
} from '../reducers/wallets';
import {
  getDisplayValues,
} from '../reducers/ui';
import {
  computePortfolio
} from '../utils/portfolio';
import {
  maskValue
} from '../utils/ui';

function calculateYield(entry, wallets) {
  if (entry.subRows) {
    let yd = null;
    for (let row of entry.subRows) {
      let perc = row.quantity / entry.quantity;
      let subYd = getYield(entry.symbol, row.wallet, wallets);
      if (subYd) {
        if (yd === null) {
            yd = 0.0;
        }
        yd += subYd * perc;
      }
    }
    return yd;
  } else {
    return getYield(entry.symbol, entry.wallet, wallets);
  }
}

function getYield(asset, wallet, wallets) {
  let w = getWallet(wallets, wallet);
  if (!w) {
    return null;
  }
  let c = w.currency.find(currency => currency.symbol == asset);
  if (!c) {
    return null;
  }
  return c.apy;
}

function computeTable(portfolio = [], prices = [], wallets = [], displayValues = true) {
  let cf = new Intl.NumberFormat(undefined, { style: 'currency', currency: 'USD' });
  let nf = new Intl.NumberFormat(undefined);
  let pnf = new Intl.NumberFormat(undefined, { style: 'percent' });

  let total = 0;
  let totalYd = 0.0;
  let results = computePortfolio(portfolio, prices);
  
  results.forEach((entry) => {
    total += entry.value;
  })

  results.forEach((entry) => {
    let yd = calculateYield(entry, wallets);
    let perc = entry.value / total;
    totalYd += yd * perc;
    entry.yield = yd === null ? "" : pnf.format(yd);
    entry.quantity = maskValue(displayValues, nf.format(entry.quantity));
    entry.value = maskValue(displayValues, cf.format(entry.value));
    if (entry.subRows) {
      entry.subRows.forEach((row) => {
        let yd = getYield(entry.symbol, row.wallet, wallets);
        row.yield = yd === null ? "" : pnf.format(yd);
        row.quantity = maskValue(displayValues, nf.format(row.quantity));
        row.value = maskValue(displayValues, cf.format(row.value));
      });
    }
  });
  return [
    results,
    maskValue(displayValues, cf.format(total)),
    pnf.format(totalYd)
  ];
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
      {
        Header: 'Yield',
        accessor: "yield",
      },
    ],
    []
  );

  const prices = useSelector(getPrices);
  const portfolios = useSelector(getPortfolios);
  const wallets = useSelector(getWallets);
  const displayValues = useSelector(getDisplayValues);

  const dispatch = useDispatch();
  useEffect(() => {
    dispatch(fetchPortfolioThunk())
  }, [dispatch])

  function getPanel(pf, prices) {
    let [data, total, yd] = computeTable(pf, prices, wallets, displayValues);
    return (
      <TabPanel key={`portfolio-tab-panel-${pf.id}`}>
        <span>Total: {total}</span>
        <span> | </span>
        <span>Yield: {yd}</span>
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
