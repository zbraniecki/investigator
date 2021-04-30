import React, { useState, useEffect } from 'react';
import Table from "../components/Table";
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import { useSelector, useDispatch } from 'react-redux'
import {
  getPrices,
  getPricesLastUpdated,
  fetchPricesThunk,
  fetchRefreshPricesThunk,
} from '../reducers/prices';
import {
  getWatchlists,
  fetchWatchlistsThunk,
} from '../reducers/watchlist';
import {
  getPortfolios,
  fetchPortfolioThunk,
} from '../reducers/portfolio';
import {
  computePortfolio
} from '../utils/portfolio';
import {
  getPrice
} from '../utils/prices';

function computeTableFromPortfolio(portfolio, prices) {
  let cf = new Intl.NumberFormat(undefined, { style: 'currency', currency: 'USD' });
  let pf = computePortfolio(portfolio, prices);
  let result = pf.map(asset => {
    let price = getPrice(prices, asset.symbol);
    return {
      "symbol": asset.symbol,
      "price": cf.format(price),
    };
  });
  return result;
}

function computeTable(wl, portfolios, prices) {
  let cf = new Intl.NumberFormat(undefined, { style: 'currency', currency: 'USD' });

  if (wl.portfolio) {
    if (portfolios.length == 0) {
      return [];
    }
    let portfolio = portfolios[0];
    return computeTableFromPortfolio(portfolio, prices);
  } else {
    let result = wl.assets
      .map(asset => {
        let price = getPrice(prices, asset);
        return {
          "symbol": asset,
          "price": price,
        };
      })
      .sort((a, b) => b.price - a.price);

    result.forEach(asset => asset.price = cf.format(asset.price));
    return result;
  }
}

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

  const prices = useSelector(getPrices);
  const pricesLastUpdated = useSelector(getPricesLastUpdated);
  const portfolios = useSelector(getPortfolios);
  const watchlists = useSelector(getWatchlists);

  const dispatch = useDispatch();
  useEffect(() => {
    dispatch(fetchPricesThunk())
    dispatch(fetchPortfolioThunk())
    dispatch(fetchWatchlistsThunk())
  }, [dispatch])

  let getPanel = (wl, prices) => {
    let dtf = new Intl.DateTimeFormat(undefined, {dateStyle: "long", timeStyle: "long"});

    let data = computeTable(wl, portfolios, prices);
    let last_updated = new Date(pricesLastUpdated);
    return (
      <TabPanel key={`watchlist-tab-panel-${wl.id}`}>
        <span>
          Last updated: {dtf.format(last_updated)}
        </span>
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
        {watchlists.map(wl => (
          <Tab key={`watchlist-tab-${wl.id}`}>{wl.name}</Tab>
        ))}
      </TabList>

      {watchlists.map(wl => getPanel(wl, prices))}
    </Tabs>
  );
}
