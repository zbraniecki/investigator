import { configureStore } from '@reduxjs/toolkit';

import uiReducer from './ui';
import portfolioReducer from './portfolio';

function saveToLocalStorage(state) {
  localStorage.setItem('theme', state.ui.theme);
}

const store = configureStore({
  reducer: {
    ui: uiReducer,
    portfolio: portfolioReducer,
  },
});

store.subscribe(() => saveToLocalStorage(store.getState()));

export default store;
