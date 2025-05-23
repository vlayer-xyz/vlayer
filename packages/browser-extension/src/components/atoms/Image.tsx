import React from "react";
import style from "./Image.module.css";

export function Image(props: React.ImgHTMLAttributes<HTMLImageElement>) {
  return (
    <img
      className={style.image}
      {...props}
      alt={props.alt ?? "User action instruction"}
    />
  );
}
