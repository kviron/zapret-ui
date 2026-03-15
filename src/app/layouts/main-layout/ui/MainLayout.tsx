import type { JSX } from "solid-js";
import { content, header, sidebar, wrapper } from "./MainLayout.styles";  
import { For } from "solid-js";

type MainLayoutProps = {
  children: JSX.Element;
};

const navItems = [
  { label: "Главная", href: "#", active: true },
  { label: "Настройки", href: "#", active: false },
  // Позже: { label: "Списки", href: "#lists" }, { label: "Настройки", href: "#settings" },
] as const;

export const MainLayout = (props: MainLayoutProps) => {
  return (
    <div class={wrapper}>
      <aside class={sidebar}>
        <nav class="app-layout__nav" aria-label="Главное меню">
          <ul>
            <For each={navItems}>
              {(item) => (
                <li>
                  <a href={item.href}>
                    {item.label}
                  </a>
                </li>
              )}
            </For>
          </ul>
        </nav>
      </aside>
        <header class={header}>
          <h1>Zapret</h1>
        </header>
        <div class={content}>{props.children}</div>
    </div>
  );
};
