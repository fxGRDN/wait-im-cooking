<script lang="ts">
    import { Drawer } from "vaul-svelte";
    import {
        getOrCreateIngredient,
        upsertInventory,
    } from "$lib/services/ingredients";

    import { fly } from "svelte/transition";

    let {
        open = $bindable(false),
        onAdd,
    }: { open: boolean; onAdd: () => void } = $props();

    let name = $state("");
    let quantity = $state<number | "">("");
    let unit = $state("");
    let saving = $state(false);
    let error = $state<string | null>(null);
    let inputRef = $state<HTMLInputElement | null>(null);

    $effect(() => {
        if (open) {
            name = "";
            quantity = "";
            unit = "";
            error = null;
        }
    });

    const submit = async () => {
        const trimmedName = name.trim();
        const trimmedUnit = unit.trim();
        const numQuantity = Number(quantity);

        if (!trimmedName) {
            error = "Name is required.";
            return;
        }

        if (!quantity || isNaN(numQuantity) || numQuantity <= 0) {
            error = "Valid quantity is required.";
            return;
        }

        saving = true;
        error = null;

        try {
            const created = await getOrCreateIngredient(
                trimmedName,
                trimmedUnit || null,
            );
            await upsertInventory(created.id, numQuantity, trimmedUnit);
            onAdd();
            // Clear fields to allow adding another
            name = "";
            quantity = "";
            unit = "";
            // Focus back on the first input
            if (inputRef) {
                inputRef.focus();
            }
        } catch (e) {
            console.error(e);
            error = "Failed to add to inventory.";
        } finally {
            saving = false;
        }
    };
</script>

<Drawer.Root bind:open>
    <Drawer.Portal>
        <Drawer.Overlay class="fixed inset-0 z-50 bg-black/40" />
        <Drawer.Content
            class="fixed bottom-0 left-0 right-0 z-50 mt-24 flex h-auto flex-col rounded-t-2xl bg-surface text-foreground shadow-lg border-t border-line outline-none pb-8 pt-4 px-4"
        >
            <div
                class="mx-auto mb-4 h-1.5 w-12 shrink-0 rounded-full bg-gray-300"
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
                        >Add to Inventory</Drawer.Title
                    >
                </div>
                <Drawer.Description class="sr-only"
                    >Add a new ingredient to your inventory. This dialog allows
                    adding multiple items without closing.</Drawer.Description
                >

                <div class="space-y-2">
                    <label class="text-sm font-medium" for="ingredient-name"
                        >Name</label
                    >
                    <input
                        id="ingredient-name"
                        bind:this={inputRef}
                        class="w-full rounded-lg border border-line bg-surface px-3 py-2 text-sm"
                        type="text"
                        bind:value={name}
                        autocomplete="off"
                        autocapitalize="words"
                        disabled={saving}
                        required
                    />
                </div>
                <div class="space-y-2">
                    <label class="text-sm font-medium" for="ingredient-quantity"
                        >Quantity</label
                    >
                    <input
                        id="ingredient-quantity"
                        class="w-full rounded-lg border border-line bg-surface px-3 py-2 text-sm"
                        type="number"
                        step="any"
                        min="0"
                        bind:value={quantity}
                        disabled={saving}
                        required
                    />
                </div>
                <div class="space-y-2">
                    <label class="text-sm font-medium" for="ingredient-unit"
                        >Unit</label
                    >
                    <input
                        id="ingredient-unit"
                        class="w-full rounded-lg border border-line bg-surface px-3 py-2 text-sm"
                        type="text"
                        bind:value={unit}
                        autocomplete="off"
                        disabled={saving}
                    />
                </div>
                {#if error}
                    <p class="text-sm text-red-600">{error}</p>
                {/if}
                <div class="flex justify-end gap-2 mt-4">
                    <Drawer.Close
                        class="px-3 py-2 text-sm rounded-lg border border-line bg-surface hover:bg-gray-50 transition"
                        disabled={saving}>Done</Drawer.Close
                    >
                    <button
                        type="submit"
                        class="px-3 py-2 text-sm rounded-lg bg-blue-600 text-white disabled:opacity-60 hover:opacity-90 transition"
                        disabled={saving || !name.trim() || !quantity}
                        >{saving ? "Saving..." : "Add"}</button
                    >
                </div>
            </form>
        </Drawer.Content>
    </Drawer.Portal>
</Drawer.Root>
