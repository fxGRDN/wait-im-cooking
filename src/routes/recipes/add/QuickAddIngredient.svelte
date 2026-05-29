<script lang="ts">
    import { Drawer } from "vaul-svelte";
    import { createIngredient } from "$lib/services/ingredients";
    import { COMMON_UNITS } from "$lib/utils";

    let {
        open = $bindable(false),
        onCreated,
        initialName = "",
    }: {
        open: boolean;
        onCreated: (ingredient: any) => void;
        initialName?: string;
    } = $props();

    let name = $state(initialName);
    let defaultUnit = $state(COMMON_UNITS[0]);
    let saving = $state(false);
    let error = $state<string | null>(null);

    $effect(() => {
        if (open) {
            name = initialName;
            defaultUnit = COMMON_UNITS[0];
            error = null;
        }
    });

    const submit = async () => {
        const trimmedName = name.trim();
        if (!trimmedName) {
            error = "Name is required.";
            return;
        }

        saving = true;
        error = null;

        try {
            const created = await createIngredient(
                trimmedName,
                defaultUnit || null,
            );
            onCreated(created);
            open = false;
        } catch (e) {
            console.error(e);
            error = "Failed to create ingredient.";
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
                class="mx-auto mb-4 h-1.5 w-12 shrink-0 rounded-full bg-line-strong"
            ></div>

            <form
                class="space-y-4"
                onsubmit={(event) => {
                    event.preventDefault();
                    submit();
                }}
            >
                <Drawer.Title class="text-lg font-semibold"
                    >New Ingredient</Drawer.Title
                >
                <Drawer.Description class="text-sm text-foreground-muted">
                    Create a new ingredient that isn't in your list yet.
                </Drawer.Description>

                <div class="space-y-2">
                    <label class="text-sm font-medium" for="new-ingredient-name"
                        >Name</label
                    >
                    <input
                        id="new-ingredient-name"
                        class="w-full rounded-lg border border-line bg-surface px-3 py-2 text-sm"
                        type="text"
                        bind:value={name}
                        disabled={saving}
                        required
                    />
                </div>

                <div class="space-y-2">
                    <label class="text-sm font-medium" for="new-ingredient-unit"
                        >Default Unit</label
                    >
                    <select
                        id="new-ingredient-unit"
                        class="w-full rounded-lg border border-line bg-surface px-3 py-2 text-sm"
                        bind:value={defaultUnit}
                        disabled={saving}
                    >
                        {#each COMMON_UNITS as u}
                            <option value={u}>{u}</option>
                        {/each}
                    </select>
                </div>

                {#if error}
                    <p class="text-sm text-danger">{error}</p>
                {/if}

                <div class="flex justify-end gap-2 mt-4">
                    <Drawer.Close
                        class="px-3 py-2 text-sm rounded-lg border border-line bg-surface hover:bg-surface-raised transition"
                    >
                        Cancel
                    </Drawer.Close>
                    <button
                        type="submit"
                        class="px-3 py-2 text-sm rounded-lg bg-accent text-background disabled:opacity-60 hover:opacity-90 transition font-bold"
                        disabled={saving || !name.trim()}
                    >
                        {saving ? "Creating..." : "Create"}
                    </button>
                </div>
            </form>
        </Drawer.Content>
    </Drawer.Portal>
</Drawer.Root>
