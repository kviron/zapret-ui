import { createSignal } from "solid-js";
import { tauriApiClient } from "../../shared/api/tauri-api-client";

export const createGreetModel = () => {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");

  const greet = async () => {
    setGreetMsg(await tauriApiClient.greet(name()));
  };

  return {
    name,
    setName,
    greetMsg,
    greet,
  };
};

