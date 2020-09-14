import React from "react";
import Head from "next/head";
import NavBar from "../components/navbar";

import "uikit/dist/js/uikit";
import "uikit/dist/js/uikit-icons";

// @ts-ignore
import style from "./_app.less";

export default function App({Component, pageProps}) {
  return (
    <div className={style.container}>
      <Head>
        <title>shr.mp</title>

        <meta charSet="utf-8"/>

        <meta name="viewport" content="width=device-width, initial-scale=1"/>

        <link rel="preload" href="https://unpkg.com/ress/dist/ress.min.css" />
        <link href="https://fonts.googleapis.com/css2?family=Raleway&display=swap" rel="stylesheet" />
      </Head>

      <NavBar/>

      <main>
        <Component {...pageProps}/>
      </main>
    </div>
  );
}
