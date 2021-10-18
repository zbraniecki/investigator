import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { fetchWallets } from '../api';

export const fetchWalletsThunk = createAsyncThunk('wallets/fetchList', fetchWallets);

export const walletsSlice = createSlice({
  name: 'wallets',
  initialState: {
    list: [],
  },
  reducers: {},
  extraReducers: {
    [fetchWalletsThunk.fulfilled]: (state, action) => {
      state.list = action.payload;
    },
  }
});

export const getWallets = state => state.wallets.list;

export function getWallet(wallets, id) {
  return wallets.find(wallet => wallet.id.toLowerCase() == id.toLowerCase());
}

export default walletsSlice.reducer;
