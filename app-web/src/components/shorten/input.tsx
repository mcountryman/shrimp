import React, { useState } from "react";
import { useDispatch } from "react-redux";
import { useCopyToClipboard } from "react-use";
import { shorten } from "../../features/shorten/actions";
import { ShortenState } from "../../features/shorten/state";
import { useAppSelector } from "../../features/store";
import ShortenButton from "./button";
// @ts-ignore
import styles from "./input.less";
import ShortenTextBox from "./textbox";

export default function ShortenInput() {
  const [long, setLong] = useState("");
  const state = useAppSelector((state) => state.shorten.state);
  const short = useAppSelector((state) => state.shorten.short);
  const dispatch = useDispatch();
  const [, copy] = useCopyToClipboard();

  return (
    <form
      className={styles.container}
      onSubmit={(evt) => {
        evt.preventDefault();

        if (state === ShortenState.Ready) {
          dispatch(shorten(long.trim()));
        }
      }}
    >
      <ShortenTextBox
        value={state === ShortenState.Shortened ? short : long}
        message={short}
        onChange={setLong}
      />

      <ShortenButton onClick={() => copy(short)} />
    </form>
  );
}
