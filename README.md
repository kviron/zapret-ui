## Zapret UI (Tauri + Solid)

**Zapret UI** — десктопное приложение на Tauri v2 + SolidJS для управления обходом DPI на базе `zapret-discord-youtube`.

Приложение не запускает `.bat`‑файлы напрямую, а использует встроенный движок (`winws.exe` + WinDivert) и свои сторы настроек.

### Как это работает

- **Движок**
  - Внутри бандла Tauri лежат ресурсы из `zapret-discord-youtube`:
    - `resources/zapret/bin` — `winws.exe`, файлы WinDivert, бинарные дампы TLS/QUIC.
  - Rust‑команда `run_default_strategy` (в `src-tauri/src/lib.rs`) ищет эти ресурсы через `app.path().resolve("zapret", BaseDirectory::Resource)` и запускает `winws.exe` через PowerShell с `Start-Process -Verb RunAs` (Windows сам спрашивает права администратора).

- **Списки (доменные и IP)**
  - В Rust есть стор `ZapretLists` (в `lib.rs`), хранящий содержимое:
    - `list_general`, `list_general_user`, `list_exclude`, `list_exclude_user`,
    - `ipset_all`, `ipset_exclude`, `ipset_exclude_user`.
  - Tauri‑команды:
    - `get_zapret_lists` — вернуть текущий стор списков.
    - `update_zapret_lists` — обновить стор списков из UI.
    - `apply_zapret_preset("original")` — загрузить дефолтные списки из встроенных файлов `zapret/lists/*.txt` (аналог оригинального репо) и записать их в стор.
  - Перед запуском `winws.exe`:
    - создаётся временная директория `zapret-lists` в `BaseDirectory::Temp`,
    - из `ZapretLists` генерируются файлы `list-*.txt` и `ipset-*.txt`,
    - в аргументах `winws.exe` используются пути именно к этим временным файлам.

- **Стратегии**
  - В Rust определён список стратегий `ZapretStrategy`:
    - `default`, `ALT`, `ALT2`, …, `ALT9` — соответствуют концепту `general.bat`, `general (ALT).bat`, и т.д.
  - Команда `get_zapret_strategies` возвращает массив стратегий с `id`, `label`, `description`.
  - Сейчас запуск использует одну “дефолтную” стратегию (набор `--filter-*` и `--dpi-desync-*` в `run_default_strategy`); выбор стратегии на фронте пока не меняет параметры запуска, но уже доступен как сущность.

### Как работает фронтенд

- **Архитектура**
  - Используется подход **Feature-Sliced Design (FSD)**:
    - `shared/api/tauri-api-client.ts` — тонкий клиент к Tauri‑командам (`getZapretLists`, `updateZapretLists`, `applyZapretPreset`, `getZapretStrategies`, `runDefaultStrategy`, `stopZapret`, `isZapretRunning`).
    - `shared/ui/*` — обёртки UI‑компонентов (кнопки, типографика, селект), которые можно переопределять по дизайну.
    - `features/zapret-lists` — фича редактирования списков:
      - `model` — загрузка/сохранение списков через `tauriApiClient`.
      - `ui/ZapretListsEditor` — форма с `textarea` для всех `list-*.txt` и `ipset-*.txt` + кнопка “Сохранить” и кнопка “Из Git (original)” для применения пресета из встроенных файлов.
    - `features/zapret-strategy` — фича выбора стратегии:
      - `model` — загрузка списка стратегий из Tauri (`getZapretStrategies`), локально выбранный `selectedId` (`"auto"` или `id` стратегии).
      - `ui/ZapretStrategySelect` — селект с опцией **“Авто”** и всеми стратегиями (`default`, `ALT…`).
    - `app/ui/App.tsx` — главный экран, который:
      - показывает заголовок и статус работы обхода (`isZapretRunning`),
      - содержит кнопки **“Запустить”** / **“Остановить”**, которые вызывают `runDefaultStrategy` и `stopZapret`,
      - рендерит `ZapretStrategySelect` и `ZapretListsEditor`.

- **Коммуникация с Tauri (по /tauri-v2)**
  - Все вызовы к Rust идут через `@tauri-apps/api/core` (`invoke`) внутри `TauriApiClient`.
  - Каждая команда оформлена как `#[tauri::command]` в `lib.rs` и внесена в `tauri::generate_handler![...]`.
  - Права и пути:
    - доступ к путям — только через `app.path().resolve(...)`,
    - прав на WinDivert добиваемся через `Start-Process -Verb RunAs`, поэтому пользователю показывается стандартный UAC‑диалог при запуске обхода.

### Как пользоваться приложением

1. Собери/запусти проект:
   - Dev: `bun run tauri dev`
   - Prod: `bun run build && bun run tauri build`
2. При первом запуске:
   - Нажми **“Из Git (original)”** в блоке “Списки Zapret”, чтобы подтянуть дефолтные списки из встроенного `zapret-upstream`.
   - При необходимости отредактируй списки в `ZapretListsEditor` и нажми **“Сохранить списки”**.
3. Выбери стратегию в блоке **“Стратегия обхода”**:
   - по умолчанию стоит **“Авто”**,
   - доступны стратегии `default`, `ALT`, `ALT2`, …, `ALT9`.
4. Нажми **“Запустить”**:
   - Windows спросит права администратора (UAC),
   - после успешного запуска статус изменится на **“активен”**.
5. Нажми **“Остановить”**, чтобы завершить процесс `winws.exe` (через `taskkill /F /IM winws.exe`).

### Что ещё можно доработать

- Привязать выбранную стратегию с фронта к разным наборам параметров `run_default_strategy` (аналог `general (ALT*).bat`).
- Хранить выбор стратегии и списков не только в памяти, но и в устойчивом хранилище (например, `tauri-plugin-store`).
- Вернуться к Ark UI Select в `shared/ui/select.tsx`, когда не будет проблем с dev/HMR, сохранив текущий интерфейс обёртки.
