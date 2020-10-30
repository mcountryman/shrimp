import { Action } from "@reduxjs/toolkit";
import { IShortenState } from "../state";

export const SHORTEN_ERROR = "SHORTEN_ERROR";

export interface IShortenErrorAction extends Action<typeof SHORTEN_ERROR> {
  error: Error;
}

export function reduceShortenError(action: IShortenErrorAction): Partial<IShortenState> {
  return {};
}
