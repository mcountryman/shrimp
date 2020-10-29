import React from "react";
import NavBar from "../components/navbar";

import "uikit/dist/js/uikit";
import "uikit/dist/js/uikit-icons";

// @ts-ignore
import style from "./_app.less";

export default function App({ Component, pageProps }) {
  return (
    <div className={style.container}>
      <NavBar />

      <main>
        <Component {...pageProps} />
      </main>
    </div>
  );
}
