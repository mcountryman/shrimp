import React, { useEffect, useRef } from "react";
import useRandomSite from "../../hooks/use_random_site";

// @ts-ignore
import styles from "./input.less";

interface IShortenTextBoxProps {
  long: string,
  short: string,
  isShort: boolean;
  onChange: (link: string) => void;
}

export default function ShortenTextBox(props: IShortenTextBoxProps) {
  const input = useRef(null);

  useEffect(() => {
    input.current?.focus()
  });

  return (
    <input
      ref={input}
      type="text"
      name="link"
      value={props.isShort ? props.short : props.long}
      readOnly={props.isShort}
      className={styles.input}
      placeholder={"your really long url"}

      onFocus={evt => evt.currentTarget.select()}
      autoFocus

      onChange={evt => props.onChange(evt.target.value)}
      onKeyPress={evt => {
        if (props.isShort) {

        }
      }}
    />
  );
}
