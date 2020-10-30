import { applyMiddleware, createStore } from "@reduxjs/toolkit";
import loggerMiddleware from "redux-logger";
import thunkMiddleware from "redux-thunk";
import { AppAction } from "./action";
import { reduceShorten } from "./shorten/reducer";
import { defaultShortenState } from "./shorten/state";
import { IAppState } from "./state";

const default_: IAppState = {
  shorten: defaultShortenState,
}

const store = createStore(
  (state: IAppState = default_, action: AppAction) => ({
    shorten: reduceShorten(state.shorten, action),
  }),
  applyMiddleware(
    thunkMiddleware,
    loggerMiddleware,
  )
);

export default store;
