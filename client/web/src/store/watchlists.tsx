import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { useDispatch } from 'react-redux';
import { fetchWatchlists } from '../api/watchlists';

export const fetchPublicWatchlistsThunk = createAsyncThunk('watchlists/fetchPublic', fetchWatchlists);

export const watchlistsSlice = createSlice({
  name: 'watchlists',
  initialState: {
    public: [],
    user: [],
  },
  reducers: {},
  extraReducers: {
    [fetchPublicWatchlistsThunk.fulfilled]: (state, action) => {
      state.public = action.payload;
    },
  },
});

export const getPublicWatchlists = (state) => state.watchlists.public;

export default watchlistsSlice.reducer;
