import type { JSX } from "solid-js";

type MainLayoutProps = {
  children: JSX.Element;
};

const navItems = [
  { label: "Главная", href: "#", active: true },
  // Позже: { label: "Списки", href: "#lists" }, { label: "Настройки", href: "#settings" },
] as const;

export const MainLayout = (props: MainLayoutProps) => {
  return (
    <div class="app-layout">
      <aside class="app-layout__sidebar">
        <nav class="app-layout__nav" aria-label="Главное меню">
          <ul class="app-layout__nav-list">
            {navItems.map((item) => (
              <li>
                <a
                  href={item.href}
                  class="app-layout__nav-link"
                  classList={{ "app-layout__nav-link--active": item.active }}
                >
                  {item.label}
                </a>
              </li>
            ))}
          </ul>
        </nav>
      </aside>
      <div class="app-layout__content">{props.children}</div>
    </div>
  );
};
