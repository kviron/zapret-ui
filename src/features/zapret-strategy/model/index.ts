import { createSignal, onMount } from "solid-js";
import { tauriApiClient, type ZapretStrategyDto } from "@/shared/api/tauri-api-client";

export type StrategyId = "auto" | string;

/** Глобальный выбор стратегии (для передачи в runDefaultStrategy из App). */
export const [selectedStrategyId, setSelectedStrategyId] = createSignal<StrategyId>("auto");

export const createZapretStrategyModel = () => {
  const [strategies, setStrategies] = createSignal<ZapretStrategyDto[]>([]);
  const [isLoading, setIsLoading] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);

  const load = async () => {
    setIsLoading(true);
    setError(null);
    try {
      const list = await tauriApiClient.getZapretStrategies();
      setStrategies(list);
    } catch {
      setError("Не удалось загрузить стратегии.");
    } finally {
      setIsLoading(false);
    }
  };

  onMount(() => {
    void load();
  });

  return {
    strategies,
    selectedId: selectedStrategyId,
    setSelectedId: setSelectedStrategyId,
    isLoading,
    error,
  };
};

