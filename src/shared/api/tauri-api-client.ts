import { Channel, invoke } from "@tauri-apps/api/core";

export interface ZapretListsDto {
  list_general: string;
  list_general_user: string;
  list_exclude: string;
  list_exclude_user: string;
  ipset_all: string;
  ipset_exclude: string;
  ipset_exclude_user: string;
}

export interface ZapretStrategyDto {
  id: string;
  label: string;
  description: string;
}

export interface DownloadProgressEvent {
  event: "Progress";
  data: { percent: number };
}

export interface DownloadCompleteEvent {
  event: "Complete";
  data: { path: string };
}

export type DownloadEvent = DownloadProgressEvent | DownloadCompleteEvent;

export class TauriApiClient {
  private async call<TResult = unknown>(
    command: string,
    args?: Record<string, unknown>,
  ): Promise<TResult> {
    return invoke<TResult>(command, args);
  }

  // Простой пример: greet(name: String) -> String
  async greet(name: string): Promise<string> {
    return this.call<string>("greet", { name });
  }

  // Пример команды с состоянием: increment() -> u32
  async increment(): Promise<number> {
    return this.call<number>("increment");
  }

  // Пример "рискованной" операции, возвращающей Result<T, E> из Rust
  async riskyOperation(): Promise<string> {
    return this.call<string>("risky_operation");
  }

  async getZapretStrategies(): Promise<ZapretStrategyDto[]> {
    return this.call<ZapretStrategyDto[]>("get_zapret_strategies");
  }

  async getZapretLists(): Promise<ZapretListsDto> {
    return this.call<ZapretListsDto>("get_zapret_lists");
  }

  async updateZapretLists(lists: ZapretListsDto): Promise<void> {
    await this.call<void>("update_zapret_lists", { lists });
  }

  async applyZapretPreset(name: string): Promise<ZapretListsDto> {
    return this.call<ZapretListsDto>("apply_zapret_preset", { name });
  }

  // Запуск основной стратегии (аналог general.bat) без .bat
  async runDefaultStrategy(options?: {
    gameFilterTcp?: string;
    gameFilterUdp?: string;
  }): Promise<void> {
    const { gameFilterTcp, gameFilterUdp } = options ?? {};
    await this.call<void>("run_default_strategy", {
      game_filter_tcp: gameFilterTcp ?? "",
      game_filter_udp: gameFilterUdp ?? "",
    });
  }

  // Остановка zapret (winws.exe) и проверка статуса
  async stopZapret(): Promise<void> {
    await this.call<void>("stop_zapret");
  }

  async isZapretRunning(): Promise<boolean> {
    return this.call<boolean>("is_zapret_running");
  }

  // Пример команды со streaming через Channel (download)
  async download(url: string, onEvent: Channel<DownloadEvent>): Promise<void> {
    await this.call<void>("download", { url, onEvent });
  }

  // Общий invoke для произвольных команд
  async invokeCommand<TResult = unknown>(
    command: string,
    args?: Record<string, unknown>,
  ): Promise<TResult> {
    return this.call<TResult>(command, args);
  }
}

export const tauriApiClient = new TauriApiClient();

