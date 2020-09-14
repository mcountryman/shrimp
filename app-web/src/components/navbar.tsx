import React from "react";
import Link from "next/link";

// @ts-ignore
import styles from "./navbar.less";

export default function NavBar() {
  return (
    <nav className={styles.nav}>
      <div className={styles.toolbarLeft}>
        <button className="uk-button uk-button-default uk-button-large">
          Login
        </button>
      </div>
      <div className={styles.toolbarRight}>
        <button className="uk-button uk-button-default uk-button-large">
          Login
        </button>
      </div>
    </nav>
  )
}
