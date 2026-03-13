import { createSignal, onMount } from "solid-js";
import { tauriApiClient, type ZapretStrategyDto } from "@/shared/api/tauri-api-client";

export type StrategyId = "auto" | string;

export const createZapretStrategyModel = () => {
  const [strategies, setStrategies] = createSignal<ZapretStrategyDto[]>([]);
  const [selectedId, setSelectedId] = createSignal<StrategyId>("auto");
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
    selectedId,
    setSelectedId,
    isLoading,
    error,
  };
};

