import { Action } from "@reduxjs/toolkit";

export const SHORTENED_LINK = "SHORTENED_LINK";

export interface IShortenedLinkAction extends Action<typeof SHORTENED_LINK> {
  short: string;
}
