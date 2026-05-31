<script lang="ts">
    import { ChefHat, Plus, Trash2 } from "@lucide/svelte";
    import type { RecipeIngredientInput, Ingredient } from "$lib/types";
    import { COMMON_UNITS } from "$lib/utils";
    import QuickAddIngredient from "../../routes/recipes/add/QuickAddIngredient.svelte";

    let {
        ingredients = $bindable([]),
        availableIngredients = [],
        onIngredientCreated,
    }: {
        ingredients: RecipeIngredientInput[];
        availableIngredients: Ingredient[];
        onIngredientCreated: (ing: Ingredient) => void;
    } = $props();

    let quickAddOpen = $state(false);
    let activeIngredientIndex = $state<number | null>(null);

    function addIngredient() {
        ingredients.push({
            ingredient_id: "",
            quantity: 1,
            unit: "",
            is_optional: false,
        });
    }

    function removeIngredient(index: number) {
        ingredients.splice(index, 1);
    }

    function handleIngredientChange(index: number, event: Event) {
        const value = (event.target as HTMLSelectElement).value;
        if (value === "new") {
            activeIngredientIndex = index;
            quickAddOpen = true;
            // Reset selection temporarily
            ingredients[index].ingredient_id = "";
        } else {
            // Always force default unit
            const ing = availableIngredients.find((a) => a.id === value);
            if (ing?.default_unit) {
                ingredients[index].unit = ing.default_unit;
            }
        }
    }

    function handleQuickAddCreated(newIng: Ingredient) {
        onIngredientCreated(newIng);
        if (activeIngredientIndex !== null) {
            ingredients[activeIngredientIndex].ingredient_id = newIng.id;
            ingredients[activeIngredientIndex].unit = newIng.default_unit || "";
        }
        activeIngredientIndex = null;
    }

    function getCompatibleUnits(unit: string | null) {
        if (!unit) return COMMON_UNITS;
        const weight = ["g", "kg"];
        const volume = ["ml", "l"];
        if (weight.includes(unit.toLowerCase())) return weight;
        if (volume.includes(unit.toLowerCase())) return volume;
        return [unit];
    }
</script>

<section class="space-y-4">
    <div class="flex items-center justify-between">
        <div
            class="flex items-center gap-2 text-foreground-muted font-bold text-sm uppercase tracking-wider"
        >
            <ChefHat size={16} />
            <span>Ingredients</span>
        </div>
        <button
            onclick={addIngredient}
            class="text-accent text-sm font-bold flex items-center gap-1 hover:bg-accent/5 px-2 py-1 rounded transition"
        >
            <Plus size={16} />
            Add Ingredient
        </button>
    </div>

    <div class="space-y-3">
        {#each ingredients as ing, i}
            <div
                class="flex gap-2 items-start bg-surface p-3 rounded-xl border border-line shadow-sm"
            >
                <div class="flex-1 space-y-2">
                    <select
                        bind:value={ing.ingredient_id}
                        onchange={(e) => handleIngredientChange(i, e)}
                        class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-accent/20"
                    >
                        <option value="">Select Ingredient</option>
                        <option value="new" class="text-accent font-bold"
                            >+ Add New Ingredient</option
                        >
                        {#each availableIngredients as a}
                            <option value={a.id}>{a.name}</option>
                        {/each}
                    </select>
                    <div class="flex gap-2">
                        <input
                            type="number"
                            step="any"
                            bind:value={ing.quantity}
                            placeholder="Qty"
                            class="w-20 bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-accent/20"
                        />
                        <select
                            bind:value={ing.unit}
                            class="flex-1 bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-accent/20"
                        >
                            {#each getCompatibleUnits(availableIngredients.find((a) => a.id === ing.ingredient_id)?.default_unit || ing.unit) as u}
                                <option value={u}>{u}</option>
                            {/each}
                        </select>
                        <label
                            class="flex items-center gap-1 px-2 whitespace-nowrap"
                        >
                            <input
                                type="checkbox"
                                bind:checked={ing.is_optional}
                                class="w-3 h-3 rounded text-accent focus:ring-accent"
                            />
                            <span class="text-xs text-foreground-muted"
                                >Opt.</span
                            >
                        </label>
                    </div>
                </div>
                <button
                    onclick={() => removeIngredient(i)}
                    class="p-2 text-foreground-subtle hover:text-danger transition"
                >
                    <Trash2 size={18} />
                </button>
            </div>
        {/each}
        {#if ingredients.length === 0}
            <div
                class="text-center py-6 border-2 border-dashed border-line rounded-xl text-foreground-subtle text-sm"
            >
                No ingredients added yet.
            </div>
        {/if}
    </div>
</section>

<QuickAddIngredient
    bind:open={quickAddOpen}
    onCreated={handleQuickAddCreated}
/>
