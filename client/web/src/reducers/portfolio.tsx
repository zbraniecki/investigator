import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { fetchPortfolios } from '../api';

export const fetchPortfolioThunk = createAsyncThunk('portfolio/fetchList', fetchPortfolios);

export const portfolioSlice = createSlice({
  name: 'portfolio',
  initialState: {
    list: [],
  },
  reducers: {},
  extraReducers: {
    [fetchPortfolioThunk.fulfilled]: (state, action) => {
      state.list = action.payload;
    },
  }
});

export const getPortfolios = state => state.portfolio.list;

export function getPortfolio(portfolios, id) {
  return portfolios.find(portfolio => portfolio.id == id);
}

export default portfolioSlice.reducer;

