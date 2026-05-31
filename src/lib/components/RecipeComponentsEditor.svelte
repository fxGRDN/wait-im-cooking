<script lang="ts">
    import { ChefHat, Plus, Trash2 } from "@lucide/svelte";
    import type { RecipeComponentInput, Recipe } from "$lib/types";

    let {
        components = $bindable([]),
        availableRecipes = [],
    }: {
        components: RecipeComponentInput[];
        availableRecipes: Recipe[];
    } = $props();

    function addComponent() {
        components.push({
            child_id: "",
            servings_needed: 1,
        });
    }

    function removeComponent(index: number) {
        components.splice(index, 1);
    }
</script>

<section class="space-y-4">
    <div class="flex items-center justify-between">
        <div
            class="flex items-center gap-2 text-foreground-muted font-bold text-sm uppercase tracking-wider"
        >
            <ChefHat size={16} />
            <span>Recipe Components</span>
        </div>
        <button
            onclick={addComponent}
            class="text-accent text-sm font-bold flex items-center gap-1 hover:bg-accent/5 px-2 py-1 rounded transition"
        >
            <Plus size={16} />
            Add Component
        </button>
    </div>

    <div class="space-y-3">
        {#each components as comp, i}
            <div
                class="flex gap-2 items-start bg-surface p-3 rounded-xl border border-line shadow-sm"
            >
                <div class="flex-1 space-y-2">
                    <select
                        bind:value={comp.child_id}
                        class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-accent/20"
                    >
                        <option value="">Select Recipe</option>
                        {#each availableRecipes as r}
                            <option value={r.id}>{r.title}</option>
                        {/each}
                    </select>
                    <div class="flex items-center gap-2">
                        <span class="text-xs text-foreground-muted"
                            >Needed servings:</span
                        >
                        <input
                            type="number"
                            step="any"
                            bind:value={comp.servings_needed}
                            placeholder="Qty"
                            class="w-20 bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-accent/20"
                        />
                    </div>
                </div>
                <button
                    onclick={() => removeComponent(i)}
                    class="p-2 text-foreground-subtle hover:text-danger transition"
                >
                    <Trash2 size={18} />
                </button>
            </div>
        {/each}
        {#if components.length === 0}
            <div
                class="text-center py-6 border-2 border-dashed border-line rounded-xl text-foreground-subtle text-sm"
            >
                No sub-recipes added.
            </div>
        {/if}
    </div>
</section>
