import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { fetchWatchlists } from '../api';

export const fetchWatchlistsThunk = createAsyncThunk('watchlist/fetchList', fetchWatchlists);

export const watchlistSlice = createSlice({
  name: 'watchlist',
  initialState: {
    list: [],
  },
  reducers: {},
  extraReducers: {
    [fetchWatchlistsThunk.fulfilled]: (state, action) => {
      state.list = action.payload;
    },
  }
});

export const getWatchlists = state => state.watchlist.list;

export default watchlistSlice.reducer;
