import React from "react";
import ShortenInput from "../components/shorten_input";

import "./index.less";

export default function IndexPage(): JSX.Element {
  return (
    <div className="shrimp-content">
      <div>
        <ShortenInput/>
      </div>
    </div>
  );
}
