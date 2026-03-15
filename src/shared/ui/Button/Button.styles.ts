import { cva } from "@styled-system/css";

/**
 * Стили кнопки по дизайну Component/Button (Pencil).
 * Варианты: Filled (Primary/Secondary), Outline (Primary/Secondary), Ghost/Link (Primary/Secondary).
 * Состояния: default, hover, active, focus, disabled.
 */
export const styles = cva({
  base: {
    display: "inline-flex",
    boxSizing: "border-box",
    position: "relative",
    alignItems: "center",
    justifyContent: "center",
    userSelect: "none",
    gap: "2",
    padding: "3",
    fontFamily: "Inter, sans-serif",
    rounded: "8px",
    cursor: "pointer",
    appearance: "none",
    border: "1px solid transparent",
    minWidth: "64px",
    transition:
      "background-color 0.15s, border-color 0.15s, opacity 0.15s, box-shadow 0.15s",
      _focusVisible: {
        outline: "none",
        boxShadow: "0 0 0px 3px rgba(59, 130, 246, 0.2)",
      },
    _disabled: {
      opacity: 0.5,
      cursor: "not-allowed",
    },
  },
  variants: {
    size: {
      small: {
        fontSize: "14px",
        padding: "6px 8px",
        gap: "1.5",
      },
      medium: {
        fontSize: "16px",
        padding: "8px 16px",
        gap: "2",
      },
      large: {
        minHeight: "52px",
        fontSize: "18px",
        padding: "12px 24px",
        gap: "2",
      },
    },
    color: {
      primary: {},
      secondary: {},
    },
    variant: {
      filled: {},
      outline: {
        bg: "transparent",
      },
      ghost: {
        bg: "transparent",
        borderColor: "transparent",
      },
      link: {
        bg: "transparent",
        textDecoration: "none",
        borderColor: "transparent",
      },
    },
  },
  compoundVariants: [
    // --- Filled Primary (дизайн: Filled → Default/Hover/Active/Focus/Disabled) ---
    {
      variant: "filled",
      color: "primary",
      css: {
        bg: "primary",
        color: "foreground",
        borderColor: "primary",
        _hover: { bg: "primary-hover" },
        _active: { bg: "primary" },
      },
    },
    // --- Filled Secondary ---
    {
      variant: "filled",
      color: "secondary",
      css: {
        bg: "card",
        color: "foreground",
        borderColor: "card",
        _hover: { bg: "card" },
        _active: { bg: "card" },
      },
    },
    // --- Outline Primary (цветная обводка и синий текст по дизайну) ---
    {
      variant: "outline",
      color: "primary",
      css: {
        bg: "transparent",
        color: "primary",
        borderWidth: "1px",
        borderStyle: "solid",
        borderColor: "primary",
        _hover: {
          borderColor: "primary-hover",
          color: "primary-hover",
        },
        _active: {
          borderColor: "primary-light",
          color: "primary-light",
        },
        _disabled: {
          borderColor: "primary",
          color: "primary",
        },
      },
    },
    // --- Outline Secondary (обводка border, focus — кольцо primary-light) ---
    {
      variant: "outline",
      color: "secondary",
      css: {
        bg: "transparent",
        color: "foreground",
        borderWidth: "1px",
        borderStyle: "solid",
        borderColor: "border",
        _hover: { borderColor: "border" },
        _active: { borderColor: "border" },
        _focusVisible: {
          outline: "2px solid",
          outlineColor: "primary-light",
          outlineOffset: "2px",
        },
      },
    },
    // --- Ghost/Link Primary ---
    {
      variant: "ghost",
      color: "primary",
      css: {
        bg: "transparent",
        color: "foreground",
        _hover: { bg: "transparent" },
        _active: { bg: "transparent" },
        _focusVisible: {
          outline: "2px solid",
          outlineColor: "primary-light",
          outlineOffset: "2px",
        },
      },
    },
    // --- Ghost/Link Secondary ---
    {
      variant: "ghost",
      color: "secondary",
      css: {
        bg: "transparent",
        color: "foreground",
        _hover: { bg: "transparent" },
        _active: { bg: "transparent" },
        _focusVisible: {
          outline: "2px solid",
          outlineColor: "primary-light",
          outlineOffset: "2px",
        },
      },
    },
    // --- Link (как ссылка) ---
    {
      variant: "link",
      color: "primary",
      css: {
        color: "primary",
        _hover: { color: "primary-hover" },
        _focusVisible: {
          outline: "2px solid",
          outlineColor: "primary-light",
          outlineOffset: "2px",
        },
      },
    },
    {
      variant: "link",
      color: "secondary",
      css: {
        color: "foreground",
        _hover: { color: "foreground-muted" },
        _focusVisible: {
          outline: "2px solid",
          outlineColor: "primary-light",
          outlineOffset: "2px",
        },
      },
    },
  ],
});
