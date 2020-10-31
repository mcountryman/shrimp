import { SerializedError } from "@reduxjs/toolkit";

export enum ShortenState {
  Ready,
  Error,
  Shortened,
}

export interface IShortenState {
  state: ShortenState,

  short?: string,
  error?: SerializedError,
}
