import { type Component, JSX, splitProps } from "solid-js";

export type HeadingProps = JSX.H1HTMLAttributes<HTMLHeadingElement>;
export type TextProps = JSX.HTMLAttributes<HTMLParagraphElement>;

export const Heading: Component<HeadingProps> = (props) => {
  const [local, others] = splitProps(props, ["class"]);
  return <h1 class={["ui-heading", local.class].filter(Boolean).join(" ")} {...others} />;
};

export const Text: Component<TextProps> = (props) => {
  const [local, others] = splitProps(props, ["class"]);
  return <p class={["ui-text", local.class].filter(Boolean).join(" ")} {...others} />;
};

