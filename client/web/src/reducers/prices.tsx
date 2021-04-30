import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { fetchPrices, fetchRefreshPrices } from '../api';

export const fetchPricesThunk = createAsyncThunk('prices/fetchList', fetchPrices);

export const fetchRefreshPricesThunk = createAsyncThunk('prices/fetchRefreshList', fetchRefreshPrices);

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
    [fetchRefreshPricesThunk.fulfilled]: (state, action) => {
      let { last_updated, price } = action.payload;
      state.list = price;
      state.last_updated = last_updated;
    },
  }
});

export const getPrices = state => state.prices.list;

export const getPricesLastUpdated = state => state.prices.last_updated;

export default pricesSlice.reducer;
