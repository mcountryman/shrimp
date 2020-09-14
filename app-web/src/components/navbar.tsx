import React from "react";
import Link from "next/link";

import "./navbar.less";

export default function NavBar() {
  return (
    <nav className="uk-navbar-container uk-navbar uk-navbar-transparent">
      <div className="uk-navbar-right">
        <ul className="uk-navbar-nav">
          <li className="uk-active">
            <button className="uk-button uk-button-default uk-button-large">
              Login
            </button>
          </li>
        </ul>
      </div>
    </nav>
  )
}
