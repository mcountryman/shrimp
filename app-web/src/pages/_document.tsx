import Document, { Html, Head, Main, NextScript } from "next/document";

if (typeof process.env.NEXT_PUBLIC_APP_URL !== "string") {
  throw new Error("Missing 'NEXT_PUBLIC_APP_URL' environment variable.");
}

export default class Shr1mpDocument extends Document {
  public render() {
    return (
      <Html>
        <Head title="shr1.mp">
          <link href="https://fonts.googleapis.com/css2?family=Raleway&display=swap" rel="stylesheet" />
        </Head>
        <body>
          <Main />
          <NextScript />
        </body>
      </Html>
    );
  }
}

