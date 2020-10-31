import { configureStore } from "@reduxjs/toolkit";
import { useDispatch, useSelector } from "react-redux";
import loggerMiddleware from "redux-logger";
import { shortenStore } from "./shorten";

export const factory = () =>
  configureStore({
    reducer: {
      shorten: shortenStore.reducer,
    },

    middleware: (getDefaultMiddleware) =>
      getDefaultMiddleware().concat(loggerMiddleware),
  });

export type AppStore = ReturnType<typeof factory>;
export type AppState = ReturnType<AppStore["getState"]>;

export const useAppSelector = <TSelected>(
  selector: (state: AppState) => TSelected,
  equalityFn?: (left: TSelected, right: TSelected) => boolean
) => useSelector(selector, equalityFn);

export const useAppDispatch = (): AppStore["dispatch"] => useDispatch();
