import React, { createRef } from "react";
import { ShortenState } from "../../features/shorten/state";
import { useAppSelector } from "../../features/store";
// @ts-ignore
import styles from "./input.less";

interface IShortenButtonProps {
  onClick: () => void;
}

export default function ShortenButton(props: IShortenButtonProps) {
  const state = useAppSelector((state) => state.shorten.state);
  const anchor = createRef<HTMLButtonElement>();

  return (
    <>
      <button
        ref={anchor}
        type="submit"
        onClick={() => {
          if (state === ShortenState.Shortened) {
            props.onClick();
          }
        }}
        className={styles.button}
      >
        {state === ShortenState.Shortened ? "copy to clipboard" : "shorten"}
      </button>
    </>
  );
}
