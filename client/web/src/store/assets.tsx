import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { useDispatch } from 'react-redux';
import { fetchAssets } from '../api/assets';

export const fetchAssetsThunk = createAsyncThunk('assets/fetch', fetchAssets);

export const assetsSlice = createSlice({
  name: 'assets',
  initialState: {
    assets: [],
  },
  reducers: {},
  extraReducers: {
    [fetchAssetsThunk.fulfilled]: (state, action) => {
      state.assets = action.payload;
    },
  },
});

export const getAssets = (state) => state.assets.assets;

export default assetsSlice.reducer;
