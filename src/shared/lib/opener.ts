import { openUrl as pluginOpenUrl, openPath, revealItemInDir } from "@tauri-apps/plugin-opener";

export async function openUrl(url: string | URL, openWith?: "inAppBrowser" | string): Promise<void> {
  await pluginOpenUrl(url, openWith);
}

export async function openFilePath(path: string, openWith?: string): Promise<void> {
  await openPath(path, openWith);
}

export async function revealInFolder(path: string | string[]): Promise<void> {
  await revealItemInDir(path);
}

