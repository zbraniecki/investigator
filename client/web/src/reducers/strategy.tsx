import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { fetchStrategies } from '../api';

export const fetchStrategyThunk = createAsyncThunk('strategy/fetchList', fetchStrategies);

export const strategySlice = createSlice({
  name: 'strategy',
  initialState: {
    list: [],
  },
  reducers: {},
  extraReducers: {
    [fetchStrategyThunk.fulfilled]: (state, action) => {
      state.list = action.payload;
    },
  }
});

export const getStrategy = state => state.strategy.list;

export default strategySlice.reducer;
