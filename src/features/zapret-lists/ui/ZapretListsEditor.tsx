import { Show } from "solid-js";
import { createZapretListsModel } from "../model";
import { Button } from "@/shared/ui/button";
import { Heading, Text } from "@/shared/ui/typography";
import { tauriApiClient } from "@/shared/api/tauri-api-client";

export const ZapretListsEditor = () => {
  const { lists, setLists, isLoading, error, save } = createZapretListsModel();

  const updateField = (field: keyof NonNullable<ReturnType<typeof lists>>) => (value: string) => {
    const current = lists();
    if (!current) return;
    setLists({ ...current, [field]: value });
  };

  return (
    <section style={{ "margin-top": "2rem" }}>
      <Heading class="text-lg">Списки Zapret</Heading>
      <Text>Редактируй домены и IP, которые используются при обходе.</Text>

      <div style={{ "margin-top": "0.75rem", display: "flex", gap: "0.5rem", "align-items": "center" }}>
        <Text>Пресеты:</Text>
        <Button
          type="button"
          disabled={isLoading()}
          onClick={async () => {
            try {
              const preset = await tauriApiClient.applyZapretPreset("original");
              setLists(preset);
            } catch {
              // ошибка уже покажется через общую ошибку загрузки/сохранения при следующем действии
            }
          }}
        >
          Из Git (original)
        </Button>
      </div>

      <Show when={error()}>
        <Text class="text-red-500">{error()}</Text>
      </Show>

      <Show when={lists()}>
        {(data) => (
          <div style={{ "margin-top": "1rem", display: "flex", "flex-direction": "column", gap: "0.75rem" }}>
            <label>
              <Text>list-general.txt</Text>
              <textarea
                rows={3}
                value={data().list_general}
                onInput={(e) => updateField("list_general")(e.currentTarget.value)}
              />
            </label>

            <label>
              <Text>list-general-user.txt</Text>
              <textarea
                rows={3}
                value={data().list_general_user}
                onInput={(e) => updateField("list_general_user")(e.currentTarget.value)}
              />
            </label>

            <label>
              <Text>list-exclude.txt</Text>
              <textarea
                rows={3}
                value={data().list_exclude}
                onInput={(e) => updateField("list_exclude")(e.currentTarget.value)}
              />
            </label>

            <label>
              <Text>list-exclude-user.txt</Text>
              <textarea
                rows={3}
                value={data().list_exclude_user}
                onInput={(e) => updateField("list_exclude_user")(e.currentTarget.value)}
              />
            </label>

            <label>
              <Text>ipset-all.txt</Text>
              <textarea
                rows={3}
                value={data().ipset_all}
                onInput={(e) => updateField("ipset_all")(e.currentTarget.value)}
              />
            </label>

            <label>
              <Text>ipset-exclude.txt</Text>
              <textarea
                rows={3}
                value={data().ipset_exclude}
                onInput={(e) => updateField("ipset_exclude")(e.currentTarget.value)}
              />
            </label>

            <label>
              <Text>ipset-exclude-user.txt</Text>
              <textarea
                rows={3}
                value={data().ipset_exclude_user}
                onInput={(e) => updateField("ipset_exclude_user")(e.currentTarget.value)}
              />
            </label>

            <Button type="button" disabled={isLoading()} onClick={() => void save()}>
              {isLoading() ? "Сохранение..." : "Сохранить списки"}
            </Button>
          </div>
        )}
      </Show>
    </section>
  );
};

