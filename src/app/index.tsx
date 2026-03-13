/* @refresh reload */
import { render } from "solid-js/web";
import { MainLayout } from "./layouts";
import App from "./ui/App";
import "./styles/app.css";

render(
  () => (
    <MainLayout>
      <App />
    </MainLayout>
  ),
  document.getElementById("root") as HTMLElement
);

