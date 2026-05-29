import { render, screen } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
import BottomNav from "../lib/components/BottomNav.svelte";
import * as state from "$app/state";

describe("BottomNav", () => {
  it("should render navigation links", () => {
    render(BottomNav);

    expect(screen.getByText("Home")).toBeInTheDocument();
    expect(screen.getByText("Inventory")).toBeInTheDocument();
    expect(screen.getByText("Recipes")).toBeInTheDocument();
    expect(screen.getByText("Settings")).toBeInTheDocument();
  });

  it("should highlight the active link based on the current path", () => {
    // @ts-ignore - Mocking readonly property
    state.page.url.pathname = "/ingredients";

    const { container } = render(BottomNav);

    // Find the active link - it should have different classes
    // Looking for "text-accent" or similar based on component logic
    const inventoryLink = screen.getByText("Inventory").closest("a");
    expect(inventoryLink).toHaveClass("text-accent");
  });
});
