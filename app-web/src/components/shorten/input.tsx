import React, {useState} from "react";
import ShortenButton from "./button";
import ShortenTextBox from "./textbox";
// @ts-ignore
import styles from "./input.less";

export default function ShortenInput() {
  const [link, setLink] = useState("");
  const [isShort, setIsShort] = useState(false);

  return (
    <div className={styles.container}>
      <ShortenTextBox
        link={link}
        isShort={isShort}
        onChange={long => {
          setLink(long);
          setIsShort(false);
        }}
      />

      <ShortenButton
        link={link}
        isShort={isShort}
        onError={console.error}
        onShortened={short => {
          setLink(short);
          setIsShort(true);
        }}
      />
    </div>
  );
}

