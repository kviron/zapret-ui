import { Show, createMemo } from "solid-js";
import { createZapretStrategyModel } from "../model";
import { UiSelect } from "@/shared/ui/select";
import { Text } from "@/shared/ui/typography";

export const ZapretStrategySelect = () => {
  const { strategies, selectedId, setSelectedId, isLoading, error } = createZapretStrategyModel();

  const options = createMemo(() => [
    {
      value: "auto",
      label: "Авто",
      description: "Автоматический выбор оптимальной стратегии",
    },
    ...strategies().map((s) => ({
      value: s.id,
      label: s.label,
      description: s.description,
    })),
  ]);

  return (
    <section style={{ "margin-top": "1.5rem" }}>
      <UiSelect
        label="Стратегия обхода"
        placeholder={isLoading() ? "Загрузка стратегий..." : "Выбери стратегию"}
        value={selectedId()}
        options={options()}
        onChange={(val) => setSelectedId(val)}
      />
      <Show when={error()}>
        <Text class="text-red-500" style={{ "margin-top": "0.5rem" }}>
          {error()}
        </Text>
      </Show>
    </section>
  );
};

