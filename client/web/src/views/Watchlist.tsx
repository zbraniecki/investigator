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
  getWallets,
  fetchWalletsThunk,
} from '../reducers/wallets';
import {
  computePortfolio
} from '../utils/portfolio';
import {
  getPrice,
  getPrice2,
} from '../utils/prices';

function computeTableFromPortfolio(portfolio, prices) {
  let cf = new Intl.NumberFormat(undefined, { style: 'currency', currency: 'USD' });
  let percf = new Intl.NumberFormat(undefined, { style: 'percent', minimumFractionDigits: 2 });
  let pf = computePortfolio(portfolio, prices);
  let result = pf.map(asset => {
    let price = getPrice2(prices, asset.symbol);
    let value = price ? price.value : 0;
    let market_cap = price ? price.market_cap : 0;
    let price_perc_24h = price ? price.price_change_percentage_24h : 0;
    let mcap_perc_24h = price ? price.market_cap_change_percentage_24h : 0;
    return {
      "symbol": asset.symbol,
      "price": value,
      "market_cap": market_cap,
      "price_perc_24h": price_perc_24h,
      "mcap_perc_24h": mcap_perc_24h,
    };
  }).sort((a, b) => b.market_cap - a.market_cap);

  result.forEach(asset => {
    asset.price = cf.format(asset.price)
    asset.price_perc_24h = percf.format(asset.price_perc_24h / 100);
    asset.mcap_perc_24h = percf.format(asset.mcap_perc_24h / 100);
  });
  return result;
}

function computeTable(wl, portfolios, prices) {
  let cf = new Intl.NumberFormat(undefined, { style: 'currency', currency: 'USD' });
  let percf = new Intl.NumberFormat(undefined, { style: 'percent', minimumFractionDigits: 2 });

  if (wl.portfolio) {
    if (portfolios.length == 0) {
      return [];
    }
    let portfolio = portfolios[0];
    return computeTableFromPortfolio(portfolio, prices);
  } else {
    let result = wl.assets
      .map(asset => {
        let price = getPrice2(prices, asset);
        let value = price ? price.value : 0;
        let market_cap = price ? price.market_cap : 0;
        let price_perc_24h = price ? price.price_change_percentage_24h : 0;
        let mcap_perc_24h = price ? price.market_cap_change_percentage_24h : 0;
        return {
          "symbol": asset,
          "price": value,
          "market_cap": market_cap,
          "price_perc_24h": price_perc_24h,
          "mcap_perc_24h": mcap_perc_24h,
        };
      }).sort((a, b) => b.market_cap - a.market_cap);

    result.forEach(asset => {
      asset.price = cf.format(asset.price)
      asset.price_perc_24h = percf.format(asset.price_perc_24h / 100);
      asset.mcap_perc_24h = percf.format(asset.mcap_perc_24h / 100);
    });
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
      {
        Header: 'Price % 24h',
        accessor: "price_perc_24h",
      },
      {
        Header: 'Mcap % 24h',
        accessor: "mcap_perc_24h",
      },
    ],
    []
  );

  const prices = useSelector(getPrices);
  const pricesLastUpdated = useSelector(getPricesLastUpdated);
  const portfolios = useSelector(getPortfolios);
  const watchlists = useSelector(getWatchlists);
  const wallets = useSelector(getWallets);

  const dispatch = useDispatch();
  const dispatchRefreshPrices = () => {
    dispatch(fetchRefreshPricesThunk())
  };
  useEffect(() => {
    dispatch(fetchPricesThunk())
    dispatch(fetchPortfolioThunk())
    dispatch(fetchWatchlistsThunk())
    dispatch(fetchWalletsThunk())
  }, [dispatch])

  let getPanel = (wl, prices, dispatchRefreshPrices) => {
    let dtf = new Intl.DateTimeFormat(undefined, {dateStyle: "long", timeStyle: "long"});

    let data = computeTable(wl, portfolios, prices);
    let last_updated = new Date(pricesLastUpdated);
    return (
      <TabPanel key={`watchlist-tab-panel-${wl.id}`}>
        <span>
          Last updated: {dtf.format(last_updated)} <button onClick={dispatchRefreshPrices}>Refresh</button>
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

      {watchlists.map(wl => getPanel(wl, prices, dispatchRefreshPrices))}
    </Tabs>
  );
}
