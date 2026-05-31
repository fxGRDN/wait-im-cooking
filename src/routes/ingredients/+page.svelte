<script lang="ts">
    import { onMount } from "svelte";
    import { getInventory } from "$lib/services/ingredients";
    import type { IngredientWithInventory } from "$lib/types";
    import AddIngredientModal from "./AddIngredientModal.svelte";
    import EditIngredientModal from "./EditIngredientModal.svelte";
    import {
        Plus,
        LayoutGrid,
        ListFilter,
        Search,
        X,
        Edit2,
        AlertTriangle,
    } from "@lucide/svelte";

    let ingredients: IngredientWithInventory[] = $state([]);
    let loading = $state(true);
    let addOpen = $state(false);
    let showAll = $state(false);
    let searchQuery = $state("");

    // Editing state
    let editingIngredient = $state<IngredientWithInventory | null>(null);
    let editOpen = $state(false);

    onMount(async () => {
        await loadData();
    });

    async function loadData() {
        try {
            ingredients = await getInventory();
        } catch (e) {
            console.error(e);
        } finally {
            loading = false;
        }
    }

    const handleAdd = async () => {
        await loadData();
    };

    onMount(() => {
        const openAdd = () => (addOpen = true);
        window.addEventListener("open-add-ingredient", openAdd);
        return () => window.removeEventListener("open-add-ingredient", openAdd);
    });

    function openEdit(ing: IngredientWithInventory) {
        editingIngredient = ing;
        editOpen = true;
    }

    let filteredIngredients = $derived(
        ingredients.filter((ing) => {
            // 1. Search filter
            if (
                searchQuery.trim() &&
                !ing.name.toLowerCase().includes(searchQuery.toLowerCase())
            ) {
                return false;
            }

            // 2. Restock filter
            if (!showAll) {
                if (!ing.restock_threshold) return true;
                if (!ing.inventory) return true;
                return ing.inventory.quantity <= ing.restock_threshold;
            }

            return true;
        }),
    );
</script>

<div class="min-h-screen bg-surface text-foreground pb-20">
    <div
        class="border-b border-line px-4 pt-[calc(1rem+env(safe-area-inset-top))] pb-4 mb-4 sticky top-0 z-10 flex flex-col gap-4 bg-surface shadow-sm"
    >
        <div class="flex justify-between items-center">
            <h1 class="text-2xl font-bold">Inventory</h1>
            <div class="flex gap-2">
                <button
                    onclick={() => (showAll = !showAll)}
                    class="p-2 rounded-lg transition {showAll
                        ? 'bg-accent text-background'
                        : 'bg-surface-sunken text-foreground-subtle'}"
                    title={showAll ? "Showing All" : "Showing Restock Only"}
                >
                    {#if showAll}
                        <LayoutGrid size={20} />
                    {:else}
                        <ListFilter size={20} />
                    {/if}
                </button>
            </div>
        </div>

        <!-- Search Bar -->
        <div class="relative">
            <Search
                size={18}
                class="absolute left-3 top-1/2 -translate-y-1/2 text-foreground-subtle"
            />
            <input
                type="text"
                bind:value={searchQuery}
                placeholder="Search ingredients..."
                class="w-full bg-surface-sunken border border-line rounded-xl pl-10 pr-10 py-2.5 text-sm outline-none focus:ring-1 focus:ring-accent/20 transition"
            />
            {#if searchQuery}
                <button
                    onclick={() => (searchQuery = "")}
                    class="absolute right-3 top-1/2 -translate-y-1/2 text-foreground-subtle hover:text-foreground transition"
                >
                    <X size={16} />
                </button>
            {/if}
        </div>
    </div>

    <div class="px-4 max-w-md mx-auto">
        {#if loading}
            <div class="space-y-4">
                {#each Array(5) as _}
                    <div
                        class="h-16 bg-surface-sunken rounded-xl animate-pulse"
                    ></div>
                {/each}
            </div>
        {:else if filteredIngredients.length === 0}
            <div
                class="text-center py-20 text-foreground-muted animate-in fade-in duration-500"
            >
                {#if searchQuery}
                    <Search size={48} class="mx-auto mb-4 opacity-10" />
                    <p>No matches found for "{searchQuery}"</p>
                {:else}
                    <ListFilter size={48} class="mx-auto mb-4 opacity-10" />
                    <p>
                        {showAll
                            ? "No inventory items found."
                            : "Nothing needs restocking!"}
                    </p>
                    {#if !showAll && ingredients.length > 0}
                        <button
                            onclick={() => (showAll = true)}
                            class="mt-2 text-sm text-accent font-bold"
                        >
                            View all items
                        </button>
                    {/if}
                {/if}
            </div>
        {:else}
            <div
                class="space-y-3 animate-in fade-in slide-in-from-bottom-4 duration-500"
            >
                {#each filteredIngredients as ingredient}
                    <button
                        onclick={() => openEdit(ingredient)}
                        class="w-full text-left p-4 bg-surface rounded-2xl border border-line shadow-sm hover:bg-surface-raised transition active:scale-[0.98] flex justify-between items-center group"
                    >
                        <div class="flex flex-col gap-0.5">
                            <span
                                class="font-bold text-foreground group-hover:text-accent transition"
                                >{ingredient.name}</span
                            >
                            {#if ingredient.restock_threshold}
                                <span
                                    class="text-[10px] text-foreground-subtle uppercase font-bold tracking-widest flex items-center gap-1"
                                >
                                    <AlertTriangle
                                        size={10}
                                        class="text-danger"
                                    />
                                    Threshold: {ingredient.restock_threshold}
                                    {ingredient.inventory?.unit || ""}
                                </span>
                            {/if}
                        </div>
                        <div class="flex items-center gap-3">
                            {#if ingredient.inventory}
                                <span
                                    class="text-xs font-bold px-2.5 py-1.5 rounded-lg
                                    {ingredient.restock_threshold &&
                                    ingredient.inventory.quantity <=
                                        ingredient.restock_threshold
                                        ? 'bg-danger/15 text-danger border border-danger/20'
                                        : 'bg-surface-sunken text-foreground-subtle border border-line'}"
                                >
                                    {ingredient.inventory.quantity.toFixed(2)}
                                    {ingredient.inventory.unit}
                                </span>
                            {/if}
                            <Edit2
                                size={16}
                                class="text-foreground-muted group-hover:text-accent transition opacity-50 group-hover:opacity-100"
                            />
                        </div>
                    </button>
                {/each}
            </div>
        {/if}
    </div>
</div>

<AddIngredientModal bind:open={addOpen} onAdd={handleAdd} />
<EditIngredientModal
    bind:open={editOpen}
    ingredient={editingIngredient}
    onUpdated={loadData}
/>
