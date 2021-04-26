import { configureStore } from '@reduxjs/toolkit';

import watchlistReducer from './reducers/watchlist';
import pricesReducer from './reducers/prices';
import portfolioReducer from './reducers/portfolio';
import strategyReducer from './reducers/strategy';

export const store = configureStore({
  reducer: {
    prices: pricesReducer,
    watchlist: watchlistReducer,
    portfolio: portfolioReducer,
    strategy: strategyReducer,
  }
});
