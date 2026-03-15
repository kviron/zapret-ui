import { css } from "@styled-system/css";


export const wrapper = css({
  display: "grid",
  gridTemplateColumns: "auto 1fr",
  gridTemplateRows: "var(--header-height) 1fr",
  gridTemplateAreas: `
    "sidebar header"
    "sidebar content"
  `,
  height: "100vh",
  backgroundColor: "background",
});

export const sidebar = css({
  maxWidth: "250px",
  backgroundColor: "sidebar",
  gridArea: "sidebar",
});

export const content = css({
  flex: 1,
  backgroundColor: "content",
  maxHeight: "calc(100vh - var(--header-height))",
  overflowY: "auto",
  gridArea: "content",
});

export const header = css({
  height: "var(--header-height)",
  backgroundColor: "header",
  gridArea: "header",
});