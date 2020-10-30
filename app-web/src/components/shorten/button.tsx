import React, { createRef } from "react";
import urlcat from "urlcat";
// @ts-ignore
import styles from "./input.less";

interface IShortenButtonProps {
  link: string,
  isShort: boolean,
  onClick: () => void,
}

export default function ShortenButton(props: IShortenButtonProps) {
  const anchor = createRef<HTMLButtonElement>();

  return (
    <>
      <button
        ref={anchor}
        type="submit"
        onClick={props.onClick}
        className={styles.button}
      >
        {props.isShort ? "copy to clipboard" : "shorten"}
      </button>
    </>
  );
}