import { configureStore } from '@reduxjs/toolkit';

import watchlistReducer from './reducers/watchlist';
import pricesReducer from './reducers/prices';
import portfolioReducer from './reducers/portfolio';
import strategyReducer from './reducers/strategy';
import walletsReducer from './reducers/wallets';
import uiReducer from './reducers/ui';

export const store = configureStore({
  reducer: {
    prices: pricesReducer,
    watchlist: watchlistReducer,
    portfolio: portfolioReducer,
    strategy: strategyReducer,
    wallets: walletsReducer,
    ui: uiReducer,
  }
});
