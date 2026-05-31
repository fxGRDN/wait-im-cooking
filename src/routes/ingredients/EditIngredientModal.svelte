<script lang="ts">
    import { Drawer } from "vaul-svelte";
    import {
        updateIngredient,
        upsertInventory,
        deleteIngredient,
    } from "$lib/services/ingredients";
    import { COMMON_UNITS } from "$lib/utils";
    import type { IngredientWithInventory } from "$lib/types";
    import { Trash2, Save, Loader2 } from "@lucide/svelte";

    let {
        open = $bindable(false),
        ingredient,
        onUpdated,
    }: {
        open: boolean;
        ingredient: IngredientWithInventory | null;
        onUpdated: () => void;
    } = $props();

    let name = $state("");
    let quantity = $state<number | "">("");
    let unit: string = $state(COMMON_UNITS[0]);
    let threshold = $state<number | "">("");
    let saving = $state(false);
    let deleting = $state(false);
    let error = $state<string | null>(null);

    $effect(() => {
        if (open && ingredient) {
            name = ingredient.name;
            quantity = ingredient.inventory?.quantity ?? "";
            unit =
                ingredient.inventory?.unit ??
                ingredient.default_unit ??
                COMMON_UNITS[0];
            threshold = ingredient.restock_threshold ?? "";
            error = null;
        }
    });

    const submit = async () => {
        if (!ingredient) return;
        const trimmedName = name.trim();
        const numQuantity = Number(quantity);
        const numThreshold = threshold === "" ? null : Number(threshold);

        if (!trimmedName) {
            error = "Name is required.";
            return;
        }

        saving = true;
        error = null;

        try {
            // Update global ingredient properties
            await updateIngredient(ingredient.id, {
                name: trimmedName,
                default_unit: unit,
                restock_threshold: numThreshold,
            });

            // Update specific inventory quantity if provided
            if (quantity !== "" && !isNaN(numQuantity)) {
                await upsertInventory(ingredient.id, numQuantity, unit);
            }

            onUpdated();
            open = false;
        } catch (e) {
            console.error(e);
            error = "Failed to update ingredient.";
        } finally {
            saving = false;
        }
    };

    const handleDelete = async () => {
        if (
            !ingredient ||
            !confirm(
                `Delete "${ingredient.name}" from library? This cannot be undone.`,
            )
        )
            return;

        deleting = true;
        try {
            await deleteIngredient(ingredient.id);
            onUpdated();
            open = false;
        } catch (e) {
            console.error(e);
            error = "Failed to delete ingredient. It might be used in recipes.";
        } finally {
            deleting = false;
        }
    };
</script>

<Drawer.Root bind:open>
    <Drawer.Portal>
        <Drawer.Overlay class="fixed inset-0 z-50 bg-black/40" />
        <Drawer.Content
            class="fixed bottom-0 left-0 right-0 z-50 mt-24 flex h-auto flex-col rounded-t-2xl bg-surface text-foreground shadow-lg border-t border-line outline-none pt-4 px-4 pb-[calc(2rem+env(safe-area-inset-bottom))]"
        >
            <div
                class="mx-auto mb-4 h-1.5 w-12 shrink-0 rounded-full bg-line-strong"
            ></div>

            <form
                class="space-y-4"
                onsubmit={(event) => {
                    event.preventDefault();
                    submit();
                }}
            >
                <div class="flex items-center justify-between">
                    <Drawer.Title class="text-lg font-semibold"
                        >Edit Ingredient</Drawer.Title
                    >
                    <button
                        type="button"
                        onclick={handleDelete}
                        disabled={deleting || saving}
                        class="p-2 text-foreground-subtle hover:text-danger transition"
                    >
                        {#if deleting}
                            <Loader2 size={20} class="animate-spin" />
                        {:else}
                            <Trash2 size={20} />
                        {/if}
                    </button>
                </div>

                <div class="space-y-2">
                    <label
                        class="text-sm font-medium"
                        for="edit-ingredient-name">Name</label
                    >
                    <input
                        id="edit-ingredient-name"
                        class="w-full rounded-lg border border-line bg-surface px-3 py-2 text-sm"
                        type="text"
                        bind:value={name}
                        disabled={saving || deleting}
                        required
                    />
                </div>

                <div class="grid grid-cols-2 gap-4">
                    <div class="space-y-2">
                        <label
                            class="text-sm font-medium"
                            for="edit-ingredient-quantity">Quantity</label
                        >
                        <input
                            id="edit-ingredient-quantity"
                            class="w-full rounded-lg border border-line bg-surface px-3 py-2 text-sm"
                            type="number"
                            step="any"
                            min="0"
                            bind:value={quantity}
                            disabled={saving || deleting}
                        />
                    </div>
                    <div class="space-y-2">
                        <label
                            class="text-sm font-medium"
                            for="edit-ingredient-unit">Unit</label
                        >
                        <select
                            id="edit-ingredient-unit"
                            class="w-full rounded-lg border border-line bg-surface px-3 py-2 text-sm"
                            bind:value={unit}
                            disabled={saving || deleting}
                        >
                            {#each COMMON_UNITS as u}
                                <option value={u}>{u}</option>
                            {/each}
                        </select>
                    </div>
                </div>

                {#if ingredient && unit !== (ingredient.inventory?.unit ?? ingredient.default_unit)}
                    <div
                        class="flex items-start gap-2 p-3 bg-warning-soft text-warning-strong rounded-xl border border-warning/20 animate-in fade-in slide-in-from-top-1 duration-200"
                    >
                        <div class="mt-0.5 shrink-0">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="14"
                                height="14"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="lucide lucide-alert-triangle"
                                ><path
                                    d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"
                                /><path d="M12 9v4" /><path
                                    d="M12 17h.01"
                                /></svg
                            >
                        </div>
                        <p
                            class="text-[10px] font-bold leading-tight uppercase tracking-tight"
                        >
                            Warning: Changing the unit will automatically update
                            all recipes using this ingredient.
                        </p>
                    </div>
                {/if}

                <div class="space-y-2">
                    <label
                        class="text-sm font-medium"
                        for="edit-ingredient-threshold">Restock Threshold</label
                    >
                    <input
                        id="edit-ingredient-threshold"
                        class="w-full rounded-lg border border-line bg-surface px-3 py-2 text-sm"
                        type="number"
                        step="any"
                        min="0"
                        placeholder="None"
                        bind:value={threshold}
                        disabled={saving || deleting}
                    />
                </div>

                {#if error}
                    <p class="text-sm text-danger">{error}</p>
                {/if}

                <div class="flex justify-end gap-2 mt-4">
                    <Drawer.Close
                        class="px-3 py-2 text-sm rounded-lg border border-line bg-surface hover:bg-surface-raised transition"
                        disabled={saving || deleting}>Cancel</Drawer.Close
                    >
                    <button
                        type="submit"
                        class="px-3 py-2 text-sm rounded-lg bg-accent text-background disabled:opacity-60 hover:opacity-90 transition font-bold flex items-center gap-2"
                        disabled={saving || deleting || !name.trim()}
                    >
                        {#if saving}
                            <Loader2 size={16} class="animate-spin" />
                            Saving...
                        {:else}
                            <Save size={16} />
                            Save Changes
                        {/if}
                    </button>
                </div>
            </form>
        </Drawer.Content>
    </Drawer.Portal>
</Drawer.Root>
