import { createSlice } from '@reduxjs/toolkit';

export const systemSlice = createSlice({
  name: 'system',
  initialState: {
    watchlists: {},
    portfolios: {},
    strategies: {},
  },
  reducers: {
    setPortfolioValues: (state, { payload }) => {
      for (let portfolio of payload) {
        state.portfolios[portfolio.id] = portfolio;
      }
    },
    setStrategyValues: (state, { payload }) => {
      for (let strategy of payload) {
        state.strategies[strategy.id] = strategy;
      }
    },
  },
  extraReducers: {},
});

export const getPortfolioValues = (state) => {
  return state.system.portfolios;
};

export const { setPortfolioValues, setStrategyValues } = systemSlice.actions;

export default systemSlice.reducer;
