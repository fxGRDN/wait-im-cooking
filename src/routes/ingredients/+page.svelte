<script lang="ts">
    import { onMount } from "svelte";
    import { getInventory } from "$lib/services/ingredients";
    import type { IngredientWithInventory } from "$lib/types";
    import AddIngredientModal from "./AddIngredientModal.svelte";

    let ingredients: IngredientWithInventory[] = $state([]);
    let loading = $state(true);
    let addOpen = $state(false);

    onMount(async () => {
        try {
            ingredients = await getInventory();
        } catch (e) {
            console.error(e);
        } finally {
            loading = false;
        }
    });

    const handleAdd = async () => {
        ingredients = await getInventory();
    };
</script>

<div class="min-h-screen bg-surface text-foreground pb-20">
    <div
        class="border-b border-line px-4 py-4 mb-4 sticky top-0 z-10 flex justify-between items-center bg-surface"
    >
        <h1 class="text-2xl font-bold">Inventory</h1>
    </div>

    <button
        type="button"
        onclick={() => (addOpen = true)}
        class="fixed bottom-24 right-6 w-14 h-14 bg-accent text-background rounded-full flex items-center justify-center shadow-lg hover:opacity-90 transition z-40"
        aria-label="Add to Inventory"
    >
        <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="5" x2="12" y2="19"></line>
            <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
    </button>

    <div class="px-4 max-w-md mx-auto">
        {#if loading}
            <div class="animate-pulse flex space-x-4">
                <div class="flex-1 space-y-4 py-1">
                    <div class="h-4 bg-gray-200 rounded w-3/4"></div>
                    <div class="space-y-2">
                        <div class="h-4 bg-gray-200 rounded"></div>
                        <div class="h-4 bg-gray-200 rounded w-5/6"></div>
                    </div>
                </div>
            </div>
        {:else if ingredients.length === 0}
            <div class="text-center py-12 text-gray-500">
                <p>No inventory items found.</p>
                <p class="mt-2 text-sm">Add items to your inventory to get started!</p>
            </div>
        {:else}
            <div
                class="bg-surface rounded-xl shadow-sm border border-line overflow-hidden"
            >
                <ul class="divide-y divide-line">
                    {#each ingredients as ingredient}
                        <li
                            class="p-4 hover:bg-gray-50 transition flex justify-between items-center"
                        >
                            <span class="font-medium">{ingredient.name}</span>
                            {#if ingredient.inventory}
                                <span
                                    class="text-xs font-semibold text-gray-700 bg-gray-200 px-2 py-1 rounded-md"
                                    >{ingredient.inventory.quantity} {ingredient.inventory.unit}</span>
                            {/if}
                        </li>
                    {/each}
                </ul>
            </div>
        {/if}
    </div>
</div>

<AddIngredientModal bind:open={addOpen} onAdd={handleAdd} />
