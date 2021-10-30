import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { useDispatch } from 'react-redux';
import { fetchStrategies } from '../api/strategies';

const USER_ID = 1;
export const fetchUserStrategiesThunk = createAsyncThunk('strategy/fetchUser', fetchStrategies.bind(undefined, USER_ID));

export const strategiesSlice = createSlice({
  name: 'strategies',
  initialState: {
    user: [],
  },
  reducers: {},
  extraReducers: {
    [fetchUserStrategiesThunk.fulfilled]: (state, action) => {
      state.user = action.payload;
    },
  },
});

export const getUserStrategies = (state) => state.strategies.user;

export default strategiesSlice.reducer;
