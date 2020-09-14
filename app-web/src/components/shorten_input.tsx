import React, {FormEvent, useEffect, useState} from "react";

import "./shorten_input.less";

export default function ShortenInput() {
  const [link, setLink] = useState("");
  const [focus, setFocus] = useState(false);
  const [shortened, setShortened] = useState(false);

  useEffect(() => {
    if (link.length === 0)
      setShortened(false);
  });

  return (
    <div className={"uk-form" + (focus ? " focus" : "")}>
      <input
        type="text"
        name="link"
        value={link}
        className="uk-input uk-form-width-large uk-form-large"
        placeholder="link"

        onBlur={_ => setFocus(false)}
        onFocus={_ => setFocus(true)}
        onChange={evt => setLink(evt.target.value)}
      />

      <button
        onClick={_ => onSubmit(link, setLink, setShortened)}
        className="uk-button uk-button-primary uk-button-large">
        Create
      </button>
    </div>
  );
}

function onSubmit(
  link: string,
  setLink: (value: string) => void,
  setShortened: (value: boolean) => void,
) {

  if (link.length === 0)
    return;

  fetch("http://f.maar.vin/api/shorten", {
    body: JSON.stringify({ url: link }),
    method: "PUT",
  })
    .then(res => res.text())
    .then(link => {
      setLink(link);
      setShortened(true);
    })
    .catch(console.error);
}
