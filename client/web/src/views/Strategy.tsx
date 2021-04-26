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

function getHoldingValue(portfolio, target, prices) {
  if (target.contains.length == 0) {
    let holding = portfolio.find(holding => holding.symbol == target.symbol);
    let price = getPrice(prices, holding.symbol);
    return holding.quantity * price;
  } else {
    let result = 0;
    for (let symbol of target.contains) {
      let holding = portfolio.find(holding => holding.symbol == symbol);
      if (holding) {
        let price = getPrice(prices, symbol);
        result += holding.quantity * price;
      }
    }
    return result;
  }
}

function computeTable(strat, port, prices) {
  let total = 0;
  let portfolio = computePortfolio(port, prices);
  portfolio.forEach(entry => {
    total += entry.value;
  });

  let pf = new Intl.NumberFormat(undefined, {style: "percent", minimumFractionDigits: 2});
  let cf = new Intl.NumberFormat(undefined, { style: 'currency', currency: 'USD' });

  let drift = 0;
  let results = strat.targets.map(target => {
    let holdingValue = getHoldingValue(portfolio, target, prices);
    let curr_per = holdingValue / total;
    let deviation = Math.abs(target.percent - curr_per);

    drift += deviation;

    let targetValue = total * target.percent;

    let delta = targetValue / holdingValue - 1;
    let usd_delta = targetValue - holdingValue;
    return {
      "symbol": target.symbol,
      "percent": target.percent,
      "current_percent": curr_per,
      "deviation": deviation,
      "delta": delta,
      "usd_delta": usd_delta,
    };
  });
  results.forEach(target => {
    target.percent = pf.format(target.percent);
    target.current_percent = pf.format(target.current_percent);
    target.deviation = target.deviation > 0.005 ?
      <strong>{pf.format(target.deviation)}</strong> :
      pf.format(target.deviation);
    target.delta = target.delta > 0.2 ?
      <strong>{pf.format(target.delta)}</strong> :
      pf.format(target.delta);
    target.usd_delta = target.usd_delta > 100 ?
      <strong>{cf.format(target.usd_delta)}</strong> :
      cf.format(target.usd_delta);
  });
  return [results, pf.format(drift / 2)];
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
        Header: 'Deviation',
        accessor: "deviation",
      },
      {
        Header: 'Delta',
        accessor: "delta",
      },
      {
        Header: 'USD Delta',
        accessor: "usd_delta",
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
