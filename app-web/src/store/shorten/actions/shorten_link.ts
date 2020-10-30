import { Action } from "@reduxjs/toolkit";

export const SHORTEN_LINK = "SHORTEN_LINK";

export interface IShortenLinkAction extends Action<typeof SHORTEN_LINK> {
  link: string;
}
