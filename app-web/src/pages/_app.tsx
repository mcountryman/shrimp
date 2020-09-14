import React from "react";
import Head from "next/head";
import NavBar from "../components/navbar";

import "uikit/dist/js/uikit";
import "uikit/dist/js/uikit-icons";
import "./_app.less";
import "../theme/_import.less";

export default function App({Component, pageProps}) {
  return (
    <div className="shr-container">
      <Head>
        <title>shr.mp</title>

        <meta charSet="utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
      </Head>

      <NavBar/>

      <main>
        <Component {...pageProps}/>
      </main>
    </div>
  );
}
