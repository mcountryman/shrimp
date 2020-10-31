import { createSlice } from "@reduxjs/toolkit";
import { shorten } from "./actions";
import { IShortenState, ShortenState } from "./state";

export const initialState: IShortenState = {
  state: ShortenState.Ready,
};

export const shortenStore = createSlice({
  name: "shorten",
  reducers: {
    reset(state) {
      state.short = undefined;
      state.error = undefined;
      state.state = ShortenState.Ready;
    },
  },
  extraReducers: (builder) =>
    builder
      .addCase(shorten.fulfilled, (state, { payload }) => {
        state.short = payload;
        state.error = undefined;
        state.state = ShortenState.Shortened;
      })
      .addCase(shorten.rejected, (state, action) => {
        state.short = undefined;
        state.error = action.error;
        state.state = ShortenState.Error;
      }),
  initialState,
});
