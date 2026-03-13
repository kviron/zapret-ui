import { createSignal, onMount } from "solid-js";
import { tauriApiClient } from "@/shared/api/tauri-api-client";
import { Button } from "@/shared/ui/button";
import { Heading, Text } from "@/shared/ui/typography";
import { ZapretListsEditor } from "@/features/zapret-lists/ui/ZapretListsEditor";
import { ZapretStrategySelect } from "@/features/zapret-strategy/ui/ZapretStrategySelect";

const App = () => {
  const [isRunning, setIsRunning] = createSignal<boolean | null>(null);
  const [isBusy, setIsBusy] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);

  const refreshStatus = async () => {
    try {
      const running = await tauriApiClient.isZapretRunning();
      setIsRunning(running);
    } catch (e) {
      console.error(e);
      setError("Не удалось проверить статус.");
    }
  };

  onMount(() => {
    void refreshStatus();
  });

  const handleStart = async () => {
    setIsBusy(true);
    setError(null);
    try {
      await tauriApiClient.runDefaultStrategy();
      await refreshStatus();
    } catch (e) {
      console.error(e);
      setError("Не удалось запустить Zapret.");
    } finally {
      setIsBusy(false);
    }
  };

  const handleStop = async () => {
    setIsBusy(true);
    setError(null);
    try {
      await tauriApiClient.stopZapret();
      await refreshStatus();
    } catch (e) {
      console.error(e);
      setError("Не удалось остановить Zapret.");
    } finally {
      setIsBusy(false);
    }
  };

  return (
    <main class="container">
      <Heading>Zapret</Heading>

      <section style={{ "margin-top": "1.5rem" }}>
        <Text>
          Управление обходом DPI для Discord/YouTube на основе zapret-discord-youtube.
        </Text>

        <div class="row" style={{ "margin-top": "1rem", gap: "0.5rem" }}>
          <Button type="button" disabled={isBusy()} onClick={() => void handleStart()}>
            {isBusy() && !isRunning() ? "Запуск..." : "Запустить"}
          </Button>
          <Button type="button" disabled={isBusy()} onClick={() => void handleStop()}>
            {isBusy() && isRunning() ? "Остановка..." : "Остановить"}
          </Button>
        </div>

        <p style={{ "margin-top": "0.75rem" }}>
          Статус:{" "}
          {isRunning() === null
            ? "проверка..."
            : isRunning()
              ? "активен"
              : "выключен"}
        </p>

        {error() && (
          <p style={{ color: "red", "margin-top": "0.5rem" }}>{error()}</p>
        )}
      </section>
      <ZapretStrategySelect />
      <ZapretListsEditor />
    </main>
  );
};

export default App;

