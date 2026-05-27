import { browser } from "$app/environment";
import { writable } from "svelte/store";

export type Theme = "light" | "dark";

const storageKey = "wait-im-cooking-theme";

function systemTheme(): Theme {
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

function readStoredTheme(): Theme | null {
    const stored = localStorage.getItem(storageKey);
    return stored === "light" || stored === "dark" ? stored : null;
}

export const theme = writable<Theme>("dark");

export function initTheme(): Theme {
    if (!browser) return "dark";

    const nextTheme = readStoredTheme() ?? systemTheme();
    theme.set(nextTheme);
    return nextTheme;
}

export function setTheme(nextTheme: Theme): void {
    theme.set(nextTheme);

    if (!browser) return;
    localStorage.setItem(storageKey, nextTheme);
}

export function toggleTheme(): void {
    theme.update((current) => {
        const nextTheme = current === "dark" ? "light" : "dark";
        if (browser) {
            localStorage.setItem(storageKey, nextTheme);
        }
        return nextTheme;
    });
}
