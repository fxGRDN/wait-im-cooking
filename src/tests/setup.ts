import "@testing-library/jest-dom";
import { vi } from "vitest";

// Mock Tauri APIs that might be called during component initialization
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
  convertFileSrc: vi.fn((path) => `http://asset.localhost/${path}`),
}));

vi.mock("@tauri-apps/api/path", () => ({
  appDataDir: vi.fn(() => Promise.resolve("/mock/app/data")),
  join: vi.fn((...args) => Promise.resolve(args.join("/"))),
}));

vi.mock("@tauri-apps/plugin-fs", () => ({
  mkdir: vi.fn(),
  readFile: vi.fn(),
  writeFile: vi.fn(),
  remove: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: vi.fn(),
}));

// SvelteKit Mocks
vi.mock("$app/state", () => ({
  page: {
    url: {
      pathname: "/",
    },
  },
}));

vi.mock("$app/paths", () => ({
  resolve: vi.fn((p) => p),
  base: "",
}));

vi.mock("$app/navigation", () => ({
  goto: vi.fn(),
}));

// Mock LocalStorage
const localStorageMock = (() => {
  let store: Record<string, string> = {};
  return {
    getItem: vi.fn((key: string) => store[key] || null),
    setItem: vi.fn((key: string, value: string) => {
      store[key] = value.toString();
    }),
    clear: vi.fn(() => {
      store = {};
    }),
    removeItem: vi.fn((key: string) => {
      delete store[key];
    }),
  };
})();

Object.defineProperty(window, "localStorage", {
  value: localStorageMock,
});
