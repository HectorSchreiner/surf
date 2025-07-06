import { Component } from "solid-js";
import { clsx } from "clsx";

export type IconProps = {
  name: string;
  class?: string;
};

export const Icon: Component<IconProps> = (props) => {
  return (
    <span class={clsx("material-symbols-outlined select-none", props.class)}>
      {props.name}
    </span>
  );
};
