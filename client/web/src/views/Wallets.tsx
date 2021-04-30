import React, { useState, useEffect } from 'react';
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import Table from "../components/Table";
import { useSelector, useDispatch } from 'react-redux'
import {
  getPrices,
} from '../reducers/prices';
import {
  getPortfolios,
} from '../reducers/portfolio';
import {
  computePortfolio
} from '../utils/portfolio';
import {
  getPrice
} from '../utils/prices';

function computeTable(portfolio, prices) {
  let cf = new Intl.NumberFormat(undefined, { style: 'currency', currency: 'USD' });
  let nf = new Intl.NumberFormat(undefined);

  let total = 0;

  let exchanges = {};

  portfolio.holdings.forEach(holding => {
    let walletName = holding.wallet || "unknown";

    if (exchanges.hasOwnProperty(walletName)) {
      exchanges[walletName].push(holding);
    } else {
      exchanges[walletName] = [holding];
    }
  });

  let result = Object.entries(exchanges).map(([key, exchange]) => {
    let value = 0;
    let subRows = exchange.map(asset => {
      let price = getPrice(prices, asset.symbol);
      value += asset.quantity * price;
      return {
        "name": "",
        "asset": asset.symbol,
        "quantity": asset.quantity,
        "value": asset.quantity * price,
      };
    }).sort((a, b) => b.value - a.value);

    subRows.forEach(item => {
      item.quantity = nf.format(item.quantity);
      item.value = cf.format(item.value);
    });
    total += value;

    return {
      "name": key,
      "value": value,
      "subRows": subRows,
    };
  }).sort((a, b) => b.value - a.value);

  result.forEach(item => {
    item.value = cf.format(item.value);
  });

  return [result, cf.format(total)];
}

export default function Strategy() {
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
        Header: 'Name',
        accessor: "name",
      },
      {
        Header: 'Asset',
        accessor: "asset",
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

  const prices = useSelector(getPrices);
  const portfolios = useSelector(getPortfolios);

  const dispatch = useDispatch();

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
