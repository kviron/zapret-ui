import { createSignal, onMount } from "solid-js";
import { tauriApiClient, type ZapretListsDto } from "@/shared/api/tauri-api-client";

export const createZapretListsModel = () => {
  const [lists, setLists] = createSignal<ZapretListsDto | null>(null);
  const [isLoading, setIsLoading] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);

  const load = async () => {
    setIsLoading(true);
    setError(null);
    try {
      const data = await tauriApiClient.getZapretLists();
      setLists(data);
    } catch {
      setError("Не удалось загрузить списки.");
    } finally {
      setIsLoading(false);
    }
  };

  const save = async () => {
    const current = lists();
    if (!current) return;
    setIsLoading(true);
    setError(null);
    try {
      await tauriApiClient.updateZapretLists(current);
    } catch {
      setError("Не удалось сохранить списки.");
    } finally {
      setIsLoading(false);
    }
  };

  onMount(() => {
    void load();
  });

  return {
    lists,
    setLists,
    isLoading,
    error,
    load,
    save,
  };
};

