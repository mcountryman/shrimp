import React, { useState } from "react";
import ShortenButton from "./button";
import ShortenTextBox from "./textbox";
// @ts-ignore
import styles from "./input.less";
import urlcat from "urlcat";

export default function ShortenInput() {
  const [long, setLong] = useState("");
  const [short, setShort] = useState("");
  const [error, setError] = useState(null);
  const [isShort, setIsShort] = useState(false);

  return (
    <form
      onSubmit={evt => {
        evt.preventDefault();

        shorten(long)
          .then(short => {
            setLong("");
            setShort(short);
            setIsShort(true);
          })
          .catch(ex => {
            setError(ex);
            setIsShort(false);
          });
      }}
      className={styles.container}
    >
      <ShortenTextBox
        long={long}
        short={short}
        isShort={isShort}
        onChange={long => {
          setLong(long);
          setIsShort(false);
        }}
      />

      <ShortenButton
        link={long}
        isShort={isShort}
        onClick={() => console.log("Test")}
      />
    </form>
  );
}

async function shorten(link: string) {
  if (link.length === 0) {
    throw new Error("Invalid");
  }

  const res = await fetch(urlcat(process.env.NEXT_PUBLIC_APP_URL, "api/shorten"), {
    body: JSON.stringify({ url: link }),
    mode: "cors",
    method: "PUT",
    headers: { "Content-Type": "application/json" }
  });

  if (!res.ok) {
    throw new Error(await res.text());
  }


  const path = await res.text();
  const full = urlcat(process.env.NEXT_PUBLIC_APP_URL, path);
  return full;
}
