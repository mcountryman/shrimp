import { createAction, createAsyncThunk, Dispatch } from "@reduxjs/toolkit";
import urlcat from "urlcat";
import { IShortenedLinkAction, SHORTENED_LINK } from "./shortened_link";
import { IShortenErrorAction, SHORTEN_ERROR } from "./shorten_error";
import { IShortenLinkAction, SHORTEN_LINK } from "./shorten_link";

export const onError = createAction<Error>("shorten/onError");
export const onShorten = createAction<string>("shorten/onShorten");
export const onShortened = createAction<string>("shorten/onShortened");
export const shorten = createAsyncThunk<string>("shorten/shorten",
  async function shorten(link: string, thunkApi) {
    // TODO: Validate
    if (link.length === 0) {
      // dispatch({
      //   type: SHORTEN_ERROR,
      //   error: new Error("Validation failed"),
      // })
    }

    const res = await fetch(urlcat(process.env.NEXT_PUBLIC_APP_URL, "api/shorten"), {
      body: JSON.stringify({ url: link }),
      mode: "cors",
      method: "PUT",
      headers: { "Content-Type": "application/json" }
    });

    if (!res.ok) {
      // dispatch({
      //   type: SHORTEN_ERROR,
      //   error: new Error(await res.text()),
      // })
    }

    return urlcat(process.env.NEXT_PUBLIC_APP_URL, await res.text());
  }
);