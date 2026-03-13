import { type Component, JSX, splitProps } from "solid-js";

export type ButtonProps = JSX.ButtonHTMLAttributes<HTMLButtonElement>;

export const Button: Component<ButtonProps> = (props) => {
  const [local, others] = splitProps(props, ["class"]);

  return (
    <button
      class={["ui-button", local.class].filter(Boolean).join(" ")}
      {...others}
    />
  );
};


