import React, { useState, useEffect } from 'react';
import Table from "../components/Table";
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import { useSelector, useDispatch } from 'react-redux'
import {
  getPrices,
  getPrice,
  fetchPricesThunk,
} from '../reducers/prices';
import {
  getWatchlists,
  fetchWatchlistsThunk,
} from '../reducers/watchlist';

function adaptDataForTable(assets = [], prices = []) {
  let cf = new Intl.NumberFormat(undefined, { style: 'currency', currency: 'USD' });

  let result = assets
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
  const watchlists = useSelector(getWatchlists);

  const dispatch = useDispatch();
  useEffect(() => {
    dispatch(fetchPricesThunk())
    dispatch(fetchWatchlistsThunk())
  }, [dispatch])

  return (
    <Tabs>
      <TabList>
        {watchlists.map(wl => (
          <Tab key={`watchlist-tab-${wl.id}`}>{wl.name}</Tab>
        ))}
      </TabList>

      {watchlists.map(wl => (
        <TabPanel key={`watchlist-tab-panel-${wl.id}`}>
          <Table
            columns={columns}
            data={adaptDataForTable(wl.assets, prices)}
          />
        </TabPanel>
      ))}
    </Tabs>
  );
}
