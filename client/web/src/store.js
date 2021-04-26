import { configureStore } from '@reduxjs/toolkit'

import watchlistReducer from './reducers/watchlist'
import pricesReducer from './reducers/prices'

export const store = configureStore({
  reducer: {
    prices: pricesReducer,
    watchlist: watchlistReducer,
  }
})
