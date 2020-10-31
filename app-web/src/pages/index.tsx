import React from "react";
import ShortenInput from "../components/shorten/input";
import Shrimp from "../components/shorten/shrimp";

// @ts-ignore
import styles from "./index.less";

export default function IndexPage(): JSX.Element {
  return (
    <div className={styles.container}>
      <div className={styles.logoContainer}>
        <Shrimp />
      </div>

      <div className={styles.inputContainer}>
        <ShortenInput />
      </div>
    </div>
  );
}
