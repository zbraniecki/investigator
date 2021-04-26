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
  getPortfolio,
  getPortfolios,
} from '../reducers/portfolio';
import {
  getStrategy,
  fetchStrategyThunk,
} from '../reducers/strategy';
import {
  computePortfolio
} from './Portfolio';

function getHolding(portfolio, symbol) {
  return portfolio.find(holding => holding.symbol == symbol);
}

function computeTable(strat, port, prices = []) {
  let total = 0;
  let portfolio = computePortfolio(port, prices);
  portfolio.forEach(entry => {
    total += entry.value;
  });

  let pf = new Intl.NumberFormat(undefined, {style: "percent", minimumFractionDigits: 2});

  let total_drift = 0;
  let results = strat.targets.map(target => {
    let holding = getHolding(portfolio, target.symbol);
    let curr_per = holding ?
      (holding.value / total) : 0;
    let drift = holding ?
      Math.abs(target.percent - curr_per) : 0;

    total_drift += drift;
    return {
      "symbol": target.symbol,
      "percent": target.percent,
      "current_percent": curr_per,
      "drift": drift,
    };
  });
  results.forEach(target => {
    target.percent = pf.format(target.percent);
    target.current_percent = pf.format(target.current_percent);
    target.drift = pf.format(target.drift);
  });
  return [results, pf.format(total_drift / 2)];
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
        Header: 'Symbol',
        accessor: "symbol",
      },
      {
        Header: 'Target %',
        accessor: "percent",
      },
      {
        Header: 'Current %',
        accessor: "current_percent",
      },
      {
        Header: 'Drift',
        accessor: "drift",
      },
    ],
    []
  );

  const prices = useSelector(getPrices);
  const strategies = useSelector(getStrategy);
  const portfolios = useSelector(getPortfolios);

  const dispatch = useDispatch();
  useEffect(() => {
    dispatch(fetchStrategyThunk())
  }, [dispatch])

  function getPanel(strat, prices) {
    let portfolio = getPortfolio(portfolios, strat.id);
    let [data, drift] = computeTable(strat, portfolio, prices);
    return (
      <TabPanel key={`strategy-tab-panel-${strat.id}`}>
        <span>Drift: {drift}</span>
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
        {strategies.map(strat => (
          <Tab key={`strategy-tab-${strat.id}`}>{strat.name}</Tab>
        ))}
      </TabList>

      {strategies.map(strat => getPanel(strat, prices))}
    </Tabs>
  );
}
