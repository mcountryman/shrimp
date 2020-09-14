import React from "react";
import useRandomSite from "../../hooks/use_random_site";

// @ts-ignore
import styles from "./input.less";

interface IShortenTextBoxProps {
  link: string;
  isShort: boolean;
  onChange: (link: string) => void;
}

export default function ShortenTextBox(props: IShortenTextBoxProps) {
  const [site,] = useRandomSite();

  return (
    <input
      type="text"
      name="link"
      value={props.link}
      className={styles.input}
      placeholder={site}

      onChange={evt => props.onChange(evt.target.value)}
    />
  );
}
