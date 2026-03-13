import { listen, emit, type EventCallback, type UnlistenFn } from "@tauri-apps/api/event";

export type TauriEventName = string;

export async function on<TPayload = unknown>(
  event: TauriEventName,
  handler: EventCallback<TPayload>,
): Promise<UnlistenFn> {
  return listen<TPayload>(event, handler);
}

export async function emitEvent<TPayload = unknown>(
  event: TauriEventName,
  payload?: TPayload,
): Promise<void> {
  await emit<TPayload>(event, payload as TPayload);
}

