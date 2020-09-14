import React from "react";
import ShortenInput from "../components/shorten/input";

// @ts-ignore
import styles from "./index.less";

export default function IndexPage(): JSX.Element {
  return (
    <div className={styles.container}>
      <div className={styles.inputContainer}>
        <ShortenInput/>
      </div>
    </div>
  );
}
