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
  computePortfolio
} from '../utils/portfolio';

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
