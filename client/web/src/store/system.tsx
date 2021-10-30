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

export const getPortfolioValue = (state, portfolioId) => {
  //XXX: This should depend on portfolio id
  if (Object.keys(state.system.portfolios).length) {
    return Object.values(state.system.portfolios)[0].value;
  } else {
    return 0;
  };
};

export const { setPortfolioValues, setStrategyValues } = systemSlice.actions;

export default systemSlice.reducer;
