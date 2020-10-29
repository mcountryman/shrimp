import React, { createRef } from "react";
import urlcat from "urlcat";
// @ts-ignore
import styles from "./input.less";

interface IShortenButtonProps {
  link: string,
  isShort: boolean,
  onShortened: (shortened: string) => void;
  onError: (ex: Error) => void;
}

export default function ShortenButton(props: IShortenButtonProps) {
  const anchor = createRef<HTMLButtonElement>();

  return (
    <>
      <button
        ref={anchor}
        disabled={props.isShort}
        className={styles.button}
        onClick={_ =>
          onSubmit(props.link)
            .then(props.onShortened)
            .catch(props.onError)
        }
      >
        shrimpify
      </button>
    </>
  );
}

function onSubmit(link: string) {
  if (link.length === 0)
    return Promise.reject("");

  return fetch(urlcat(process.env.NEXT_PUBLIC_APP_URL, "api/shorten"), {
    body: JSON.stringify({ url: link }),
    method: "PUT",
  })
    .then(res => res.text());
}