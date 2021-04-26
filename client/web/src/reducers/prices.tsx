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

export default pricesSlice.reducer;

