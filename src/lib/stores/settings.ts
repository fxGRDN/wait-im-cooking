import { browser } from "$app/environment";
import { writable } from "svelte/store";

export interface Settings {
  fontScale: number; // 80 to 150
  leftHandedMode: boolean;
  pickedVideo: string | null;
  pickedMusic: string | null;
}

const storageKey = "wait-im-cooking-settings";
const defaultSettings: Settings = {
  fontScale: 100,
  leftHandedMode: false,
  pickedVideo: null,
  pickedMusic: null,
};

function readStoredSettings(): Settings {
  if (!browser) return defaultSettings;
  const stored = localStorage.getItem(storageKey);
  if (!stored) return defaultSettings;
  try {
    return { ...defaultSettings, ...JSON.parse(stored) };
  } catch (e) {
    return defaultSettings;
  }
}

export const settings = writable<Settings>(readStoredSettings());

export function initSettings() {
  if (!browser) return;
  settings.subscribe((value) => {
    localStorage.setItem(storageKey, JSON.stringify(value));
    applySettings(value);
  });
}

function applySettings(value: Settings) {
  if (!browser) return;
  const baseSize = 15;
  const scaledSize = (baseSize * value.fontScale) / 100;
  document.documentElement.style.setProperty(
    "--font-size-base",
    `${scaledSize}px`,
  );
}

export function updateFontScale(scale: number) {
  settings.update((s) => ({ ...s, fontScale: scale }));
}

export function updateLeftHandedMode(enabled: boolean) {
  settings.update((s) => ({ ...s, leftHandedMode: enabled }));
}

export function updatePickedVideo(path: string | null) {
  settings.update((s) => ({ ...s, pickedVideo: path }));
}

export function updatePickedMusic(path: string | null) {
  settings.update((s) => ({ ...s, pickedMusic: path }));
}
