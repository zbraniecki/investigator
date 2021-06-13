import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { fetchPortfolios } from '../api/markets';

export const fetchPortfoliosThunk = createAsyncThunk('markets/fetchPortfolios', fetchPortfolios);

export const marketsSlice = createSlice({
  name: 'markets',
  initialState: {
    portfolios: [],
  },
  reducers: {},
  extraReducers: {
    [fetchPortfoliosThunk.fulfilled]: (state, action) => {
      state.portfolios = action.payload;
    },
  },
});

export const getPortfolios = (state) => state.markets.portfolios;

export default marketsSlice.reducer;
