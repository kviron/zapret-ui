import type { Component } from "solid-js";
import { splitProps } from "solid-js";

export interface SelectOption {
  value: string;
  label: string;
  description?: string;
}

export interface SelectProps {
  label?: string;
  placeholder?: string;
  value?: string;
  options: SelectOption[];
  onChange?: (value: string) => void;
}

export const UiSelect: Component<SelectProps> = (props) => {
  const [local] = splitProps(props, ["label", "placeholder", "value", "options", "onChange"]);

  return (
    <div class="ui-select">
      {local.label && <label class="ui-select-label">{local.label}</label>}
      <select
        class="ui-select-native"
        value={local.value}
        onChange={(e) => local.onChange?.(e.currentTarget.value)}
      >
        {!local.value && (
          <option value="" disabled>
            {local.placeholder ?? "Выбери вариант"}
          </option>
        )}
        {local.options.map((opt) => (
          <option value={opt.value}>{opt.label}</option>
        ))}
      </select>
    </div>
  );
};

