import { createAsyncThunk } from "@reduxjs/toolkit";
import urlcat from "urlcat";
import { EMPTY_URL, INVALID_URL } from "./errors";

const validScheme = new RegExp(process.env.NEXT_PUBLIC_VALID_SCHEME);

export function tryGetValidUrl(link: string): string {
  if (link.length === 0) {
    throw new Error(EMPTY_URL);
  }

  let url: URL;
  try {
    url = new URL(link);
  } catch (ex) {
    try {
      url = new URL("https://" + link);
    } catch (ex) {
      throw new Error(INVALID_URL);
    }
  }

  const scheme = url.protocol.slice(0, -1);
  if (!scheme.match(validScheme)) {
    throw new Error(INVALID_URL);
  }

  return url.toString();
}

export const shorten = createAsyncThunk(
  "shorten/shorten",
  async function shorten(link: string) {
    const url = tryGetValidUrl(link);
    const res = await fetch(
      urlcat(process.env.NEXT_PUBLIC_APP_URL, "api/shorten"),
      {
        body: JSON.stringify({ url }),
        mode: "cors",
        method: "PUT",
        headers: { "Content-Type": "application/json" },
      }
    );

    if (!res.ok) {
      throw new Error(await res.text());
    }

    return urlcat(process.env.NEXT_PUBLIC_APP_URL, await res.text());
  }
);
