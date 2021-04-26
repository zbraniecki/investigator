import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { fetchPrices } from '../api';

export const fetchPricesThunk = createAsyncThunk('prices/fetchList', fetchPrices);

export const pricesSlice = createSlice({
  name: 'prices',
  initialState: {
    list: [],
    last_updated: null,
  },
  reducers: {},
  extraReducers: {
    [fetchPricesThunk.fulfilled]: (state, action) => {
      let { last_updated, price } = action.payload;
      state.list = price;
      state.last_updated = last_updated;
    },
  }
});

export const getPrices = state => state.prices.list;

export function getPrice(prices, symbol) {
  if (symbol == "usd") {
    return 1;
  }

  for (let price of prices) {
    if (price.pair[0] == symbol) {
      return price.value;
    }
  }
  return null;
}

export default pricesSlice.reducer;

