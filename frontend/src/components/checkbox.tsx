import React, { ComponentProps } from "react";
import "./checkbox.css";

interface TProps extends ComponentProps<"input"> {
  checked: ComponentProps<"input">["checked"];
  onChange: ComponentProps<"input">["onChange"];
}

function Checkbox(p: TProps) {
  return (
    <div className="checkbox">
      <input
        type="checkbox"
        className="checkbox__trigger"
        onChange={p.onChange}
        checked={p.checked}
      />

      <div className="checkbox__body">
        <div className="checkbox__mark" />
      </div>

      <span className="checkbox__text">{p.children}</span>
    </div>
  );
}

export { Checkbox };
