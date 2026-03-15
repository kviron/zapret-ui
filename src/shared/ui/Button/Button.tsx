import { type Component, JSX, splitProps } from "solid-js";
import { type RecipeVariantProps, cx } from "@styled-system/css";
import { styles } from "./Button.styles";

type StyleVariants = RecipeVariantProps<typeof styles>;

type ButtonCommonProps = {
  startIcon?: JSX.Element;
  endIcon?: JSX.Element;
};

type ButtonAsButton = StyleVariants &
  JSX.ButtonHTMLAttributes<HTMLButtonElement> & {
    variant?: "filled" | "outline" | "ghost";
  };

type ButtonAsLink = StyleVariants &
  JSX.AnchorHTMLAttributes<HTMLAnchorElement> & {
    variant: "link";
    href: string;
  };

export type ButtonProps = ButtonCommonProps & (ButtonAsButton | ButtonAsLink);

export const Button: Component<ButtonProps> = (props) => {
  const [local, others] = splitProps(props, [
    "class",
    "size",
    "color",
    "variant",
    "type",
    "startIcon",
    "endIcon",
  ]);
  const isLink = local.variant === "link";

  const recipeClass = styles({
    size: local?.size ?? "medium",
    color: local?.color ?? "primary",
    variant: local?.variant ?? "filled",
  });
  const classList = cx(recipeClass, local.class);

  return isLink ? (
    <a
      href={(props as ButtonAsLink).href ?? "#"}
      class={classList}
      {...(others as JSX.AnchorHTMLAttributes<HTMLAnchorElement>)}
    >
      {local.startIcon}
      <span>{props.children}</span>
      {local.endIcon}
    </a>
  ) : (
    <button
      type={(local?.type as JSX.ButtonHTMLAttributes<HTMLButtonElement>["type"]) ?? "button"}
      class={classList}
      {...(others as JSX.ButtonHTMLAttributes<HTMLButtonElement>)}
    >
      {local.startIcon}
      <span>{props.children}</span>
      {local.endIcon}
    </button>
  );
};


