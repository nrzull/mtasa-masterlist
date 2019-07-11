import "./header.css";
import React from "react";

function Header() {
  return (
    <div className="header">
      <div className="container">
        <div className="header__logo">
          <a target="_blank" rel="noopener" href="https://mtasa.com/">
            <div className="header__logo-image" />
          </a>

          <div className="header__logo-text">
            <span>m</span>
            <span>a</span>
            <span>s</span>
            <span>t</span>
            <span>e</span>
            <span>r</span>
            <span>l</span>
            <span>i</span>
            <span>s</span>
            <span>t</span>
          </div>
        </div>
      </div>
    </div>
  );
}

export { Header };
