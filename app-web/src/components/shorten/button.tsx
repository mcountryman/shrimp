import React, { createRef } from "react";
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

  return new Promise((resolve, reject) => {
    setTimeout(() => {

      let result = [];
      let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

      for (let i = 0; i < 5; i++) {
        const chIndex = Math.floor(Math.random() * characters.length);
        const ch = characters[chIndex];

        result.push(ch);
      }

      resolve(process.env.NEXT_PUBLIC_APP_URL + " " + result.join(""));
    }, Math.random() * 1000);
  });
  // return fetch("http://f.maar.vin/api/shorten", {
  //   body: JSON.stringify({url: link}),
  //   method: "PUT",
  // })
  //   .then(res => res.text());
}