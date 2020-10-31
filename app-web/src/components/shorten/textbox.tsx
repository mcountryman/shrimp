import React, { useEffect, useRef } from "react";
import { shortenStore } from "../../features/shorten";
import { shorten } from "../../features/shorten/actions";
import { ShortenState } from "../../features/shorten/state";
import { useAppDispatch, useAppSelector } from "../../features/store";
// @ts-ignore
import styles from "./input.less";

interface IProps {
  value: string;
  message?: string;
  onChange: (value: string) => void;
}

export default function ShortenTextBox(props: IProps) {
  const input = useRef(null);
  const dispatch = useAppDispatch();
  const reset = () => {
    if (props.message) {
      dispatch(shortenStore.actions.reset());
      props.onChange("");
    }
  };

  useEffect(() => {
    input.current?.focus();
  });

  return (
    <input
      ref={input}
      type="text"
      name="link"
      
      value={props.message || props.value}
      className={styles.input}
      placeholder={"your really long url"}

      onFocus={(evt) => evt.currentTarget.select()}
      autoFocus

      onPaste={reset}
      onBeforeInput={reset}

      onChange={(evt) => props.onChange(evt.target.value)}
      onKeyDown={(evt) => {
        console.log(evt.key)
        switch (evt.key) {
          case "Delete":
          case "Backspace":
            reset();
            break;
        }
      }}
    />
  );
}
