<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import {
        createRecipe,
        getRecipes,
        getTags,
        createTag,
    } from "$lib/services/recipes";
    import { getIngredients } from "$lib/services/ingredients";
    import type {
        RecipeInput,
        RecipeIngredientInput,
        RecipeComponentInput,
        StepInput,
        Ingredient,
        Recipe,
        Tag,
    } from "$lib/types";
    import {
        Plus,
        Trash2,
        ChevronLeft,
        Save,
        Clock,
        ChefHat,
        Info,
        Tag as TagIcon,
        Camera,
        X as XIcon,
    } from "lucide-svelte";
    import QuickAddIngredient from "./QuickAddIngredient.svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { saveImages } from "$lib/utils";

    let title = $state("");
    let description = $state("");
    let servings = $state<number | null>(null);
    let prepTime = $state<number | null>(null);
    let cookTime = $state<number | null>(null);
    let isFavourite = $state(false);
    let coverImage = $state<string | null>(null);

    let recipeIngredients = $state<RecipeIngredientInput[]>([]);
    let recipeComponents = $state<RecipeComponentInput[]>([]);
    let recipeSteps = $state<StepInput[]>([]);
    let tagIds = $state<string[]>([]);

    let availableIngredients = $state<Ingredient[]>([]);
    let availableRecipes = $state<Recipe[]>([]);
    let availableTags = $state<Tag[]>([]);
    let newTagName = $state("");

    let saving = $state(false);
    let error = $state<string | null>(null);

    // Quick add state
    let quickAddOpen = $state(false);
    let activeIngredientIndex = $state<number | null>(null);

    onMount(async () => {
        try {
            [availableIngredients, availableRecipes, availableTags] =
                await Promise.all([getIngredients(), getRecipes(), getTags()]);
        } catch (e) {
            console.error(e);
        }
    });

    const addIngredient = () => {
        recipeIngredients.push({
            ingredient_id: "",
            quantity: 1,
            unit: "",
            is_optional: false,
        });
    };

    const removeIngredient = (index: number) => {
        recipeIngredients.splice(index, 1);
    };

    const handleIngredientChange = (index: number, event: Event) => {
        const value = (event.target as HTMLSelectElement).value;
        if (value === "new") {
            activeIngredientIndex = index;
            quickAddOpen = true;
            // Reset selection temporarily
            recipeIngredients[index].ingredient_id = "";
        } else {
            // Auto-fill unit if it's empty
            const ing = availableIngredients.find((a) => a.id === value);
            if (ing?.default_unit && !recipeIngredients[index].unit) {
                recipeIngredients[index].unit = ing.default_unit;
            }
        }
    };

    const onIngredientCreated = (newIng: Ingredient) => {
        availableIngredients = [...availableIngredients, newIng].sort((a, b) =>
            a.name.localeCompare(b.name),
        );
        if (activeIngredientIndex !== null) {
            recipeIngredients[activeIngredientIndex].ingredient_id = newIng.id;
            recipeIngredients[activeIngredientIndex].unit =
                newIng.default_unit || "";
        }
        activeIngredientIndex = null;
    };

    const addStep = () => {
        recipeSteps.push({
            step_order: recipeSteps.length + 1,
            step_type: "cook",
            description: "",
            duration_min: null,
        });
    };

    const removeStep = (index: number) => {
        recipeSteps.splice(index, 1);
        // Re-order steps
        recipeSteps.forEach((s, i) => (s.step_order = i + 1));
    };

    const addComponent = () => {
        recipeComponents.push({
            child_id: "",
            servings_needed: 1,
        });
    };

    const removeComponent = (index: number) => {
        recipeComponents.splice(index, 1);
    };

    const handleCreateTag = async () => {
        const name = newTagName.trim();
        if (!name) return;
        try {
            const tag = await createTag(name);
            availableTags.push(tag);
            tagIds.push(tag.id);
            newTagName = "";
        } catch (e) {
            console.error(e);
        }
    };

    const toggleTag = (id: string) => {
        const idx = tagIds.indexOf(id);
        if (idx === -1) tagIds.push(id);
        else tagIds.splice(idx, 1);
    };

    const handleSave = async () => {
        if (!title.trim()) {
            error = "Title is required";
            return;
        }

        saving = true;
        error = null;

        try {
            const data: RecipeInput = {
                title: title.trim(),
                description: description.trim() || null,
                servings,
                prep_time: prepTime,
                cook_time: cookTime,
                is_favourite: isFavourite,
                cover_image: coverImage,
            };

            // Filter out empty ingredients/components/steps
            const filteredIngredients = recipeIngredients.filter(
                (i) => i.ingredient_id !== "",
            );
            const filteredComponents = recipeComponents.filter(
                (c) => c.child_id !== "",
            );
            const filteredSteps = recipeSteps.filter(
                (s) => s.description.trim() !== "",
            );

            await createRecipe(
                data,
                filteredIngredients,
                filteredComponents,
                filteredSteps,
                tagIds,
            );
            goto("/recipes");
        } catch (e) {
            console.error(e);
            error = e instanceof Error ? e.message : "Failed to create recipe";
        } finally {
            saving = false;
        }
    };

    async function pickCover() {
        const savedPath = await saveImages("recipe_covers");
        if (savedPath) {
            console.log(`Cover image set to ${coverImage}`);
            coverImage = savedPath[0];
        }
    }
</script>

<div class="min-h-screen bg-surface text-foreground pb-20">
    <div
        class="border-b border-line px-4 pt-[calc(1rem+env(safe-area-inset-top))] pb-4 mb-4 sticky top-0 z-10 flex justify-between items-center bg-surface"
    >
        <div class="flex items-center gap-2">
            <a
                href="/recipes"
                class="p-2 -ml-2 hover:bg-surface-sunken rounded-full transition"
            >
                <ChevronLeft size={24} />
            </a>
            <h1 class="text-xl font-bold">New Recipe</h1>
        </div>
        <button
            onclick={handleSave}
            disabled={saving || !title.trim()}
            class="flex items-center gap-2 bg-accent text-background px-4 py-2 rounded-lg font-bold disabled:opacity-50 hover:opacity-90 transition"
        >
            <Save size={18} />
            {saving ? "Saving..." : "Save"}
        </button>
    </div>

    <div class="px-4 max-w-2xl mx-auto space-y-8">
        {#if error}
            <div
                class="bg-danger-soft text-danger p-4 rounded-xl border border-danger-edge text-sm"
            >
                {error}
            </div>
        {/if}

        <!-- Basic Info -->
        <section class="space-y-4">
            <div
                class="flex items-center gap-2 text-foreground-muted font-bold text-sm uppercase tracking-wider"
            >
                <Info size={16} />
                <span>General Information</span>
            </div>
            <div
                class="bg-surface p-4 rounded-xl border border-line space-y-4 shadow-sm"
            >
                <!-- Image Picker -->
                <div class="space-y-2">
                    <label class="text-sm font-medium">Cover Image</label>
                    {#if coverImage}
                        <div
                            class="relative w-full aspect-video rounded-xl overflow-hidden border border-line"
                        >
                            <img
                                src={convertFileSrc(coverImage)}
                                alt="Cover preview"
                                class="w-full h-full object-cover"
                            />
                            <button
                                onclick={() => (coverImage = null)}
                                class="absolute top-2 right-2 bg-black/50 text-white p-1.5 rounded-full hover:bg-black/70 transition"
                            >
                                <XIcon size={16} />
                            </button>
                        </div>
                    {:else}
                        <button
                            onclick={pickCover}
                            class="w-full aspect-video rounded-xl border-2 border-dashed border-line flex flex-col items-center justify-center text-foreground-subtle gap-2 hover:bg-surface-raised transition"
                        >
                            <Camera size={32} strokeWidth={1.5} />
                            <span
                                class="text-xs font-bold uppercase tracking-wider"
                                >Add Cover Photo</span
                            >
                        </button>
                    {/if}
                </div>

                <div class="space-y-1">
                    <label for="title" class="text-sm font-medium"
                        >Recipe Title</label
                    >
                    <input
                        id="title"
                        bind:value={title}
                        placeholder="e.g. Classic Carbonara"
                        class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 focus:ring-2 focus:ring-accent/20 focus:border-accent outline-none transition"
                    />
                </div>
                <div class="space-y-1">
                    <label for="description" class="text-sm font-medium"
                        >Description</label
                    >
                    <textarea
                        id="description"
                        bind:value={description}
                        placeholder="Tell a bit about this dish..."
                        rows="3"
                        class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 focus:ring-2 focus:ring-accent/20 focus:border-accent outline-none transition resize-none"
                    ></textarea>
                </div>
                <div class="grid grid-cols-3 gap-4">
                    <div class="space-y-1">
                        <label for="servings" class="text-sm font-medium"
                            >Servings</label
                        >
                        <input
                            id="servings"
                            type="number"
                            bind:value={servings}
                            class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 outline-none"
                        />
                    </div>
                    <div class="space-y-1">
                        <label for="prep" class="text-sm font-medium"
                            >Prep (m)</label
                        >
                        <input
                            id="prep"
                            type="number"
                            bind:value={prepTime}
                            class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 outline-none"
                        />
                    </div>
                    <div class="space-y-1">
                        <label for="cook" class="text-sm font-medium"
                            >Cook (m)</label
                        >
                        <input
                            id="cook"
                            type="number"
                            bind:value={cookTime}
                            class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 outline-none"
                        />
                    </div>
                </div>
                <label
                    class="flex items-center gap-2 cursor-pointer select-none"
                >
                    <input
                        type="checkbox"
                        bind:checked={isFavourite}
                        class="w-4 h-4 rounded text-accent focus:ring-accent"
                    />
                    <span class="text-sm font-medium">Mark as favourite</span>
                </label>
            </div>
        </section>

        <!-- Tags -->
        <section class="space-y-4">
            <div
                class="flex items-center gap-2 text-foreground-muted font-bold text-sm uppercase tracking-wider"
            >
                <TagIcon size={16} />
                <span>Tags</span>
            </div>
            <div
                class="bg-surface p-4 rounded-xl border border-line shadow-sm space-y-4"
            >
                <div class="flex flex-wrap gap-2">
                    {#each availableTags as tag}
                        <button
                            onclick={() => toggleTag(tag.id)}
                            class="px-3 py-1 rounded-full text-xs font-medium border transition {tagIds.includes(
                                tag.id,
                            )
                                ? 'bg-accent text-background border-accent'
                                : 'bg-surface-sunken text-foreground-muted border-line hover:border-accent/50'}"
                        >
                            {tag.name}
                        </button>
                    {/each}
                    {#if availableTags.length === 0}
                        <p class="text-xs text-foreground-subtle italic">
                            No tags created yet.
                        </p>
                    {/if}
                </div>
                <div class="flex gap-2">
                    <input
                        type="text"
                        bind:value={newTagName}
                        placeholder="New tag name..."
                        class="flex-1 bg-surface-sunken border border-line rounded-lg px-3 py-1.5 text-sm outline-none"
                        onkeydown={(e) =>
                            e.key === "Enter" && handleCreateTag()}
                    />
                    <button
                        onclick={handleCreateTag}
                        class="px-3 py-1.5 bg-surface-sunken text-foreground-muted rounded-lg text-sm font-bold hover:bg-surface-raised transition"
                    >
                        Add
                    </button>
                </div>
            </div>
        </section>

        <!-- Ingredients -->
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
                {#each recipeIngredients as ing, i}
                    <div
                        class="flex gap-2 items-start bg-surface p-3 rounded-xl border border-line shadow-sm"
                    >
                        <div class="flex-1 space-y-2">
                            <select
                                bind:value={ing.ingredient_id}
                                onchange={(e) => handleIngredientChange(i, e)}
                                class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none"
                            >
                                <option value="">Select Ingredient</option>
                                <option
                                    value="new"
                                    class="text-accent font-bold"
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
                                    class="w-20 bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none"
                                />
                                <input
                                    type="text"
                                    bind:value={ing.unit}
                                    placeholder="Unit"
                                    class="flex-1 bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none"
                                />
                                <label
                                    class="flex items-center gap-1 px-2 whitespace-nowrap"
                                >
                                    <input
                                        type="checkbox"
                                        bind:checked={ing.is_optional}
                                        class="w-3 h-3 rounded"
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
                {#if recipeIngredients.length === 0}
                    <div
                        class="text-center py-6 border-2 border-dashed border-line rounded-xl text-foreground-subtle text-sm"
                    >
                        No ingredients added yet.
                    </div>
                {/if}
            </div>
        </section>

        <!-- Components (Sub-recipes) -->
        <section class="space-y-4">
            <div class="flex items-center justify-between">
                <div
                    class="flex items-center gap-2 text-foreground-muted font-bold text-sm uppercase tracking-wider"
                >
                    <Plus size={16} />
                    <span>Sub-recipes</span>
                </div>
                <button
                    onclick={addComponent}
                    class="text-accent text-sm font-bold flex items-center gap-1 hover:bg-accent/5 px-2 py-1 rounded transition"
                >
                    <Plus size={16} />
                    Add Sub-recipe
                </button>
            </div>
            <div class="space-y-3">
                {#each recipeComponents as comp, i}
                    <div
                        class="flex gap-2 items-start bg-surface p-3 rounded-xl border border-line shadow-sm"
                    >
                        <div class="flex-1 space-y-2">
                            <select
                                bind:value={comp.child_id}
                                class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none"
                            >
                                <option value="">Select Recipe</option>
                                {#each availableRecipes as r}
                                    <option value={r.id}>{r.title}</option>
                                {/each}
                            </select>
                            <div class="flex items-center gap-2">
                                <span class="text-xs text-foreground-muted"
                                    >Need servings:</span
                                >
                                <input
                                    type="number"
                                    bind:value={comp.servings_needed}
                                    class="w-20 bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none"
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
            </div>
        </section>

        <!-- Steps -->
        <section class="space-y-4 pb-12">
            <div class="flex items-center justify-between">
                <div
                    class="flex items-center gap-2 text-foreground-muted font-bold text-sm uppercase tracking-wider"
                >
                    <Clock size={16} />
                    <span>Cooking Steps</span>
                </div>
                <button
                    onclick={addStep}
                    class="text-accent text-sm font-bold flex items-center gap-1 hover:bg-accent/5 px-2 py-1 rounded transition"
                >
                    <Plus size={16} />
                    Add Step
                </button>
            </div>
            <div class="space-y-4">
                {#each recipeSteps as step, i}
                    <div
                        class="bg-surface p-4 rounded-xl border border-line shadow-sm space-y-3 relative"
                    >
                        <div class="flex justify-between items-center">
                            <span
                                class="bg-surface-sunken text-foreground-muted w-6 h-6 rounded-full flex items-center justify-center text-xs font-bold"
                            >
                                {step.step_order}
                            </span>
                            <div class="flex items-center gap-2">
                                <select
                                    bind:value={step.step_type}
                                    class="text-xs bg-surface-sunken border border-line rounded-md px-2 py-1 outline-none"
                                >
                                    <option value="prep">Prep</option>
                                    <option value="cook">Cook</option>
                                </select>
                                <button
                                    onclick={() => removeStep(i)}
                                    class="p-1 text-foreground-subtle hover:text-danger transition"
                                >
                                    <Trash2 size={16} />
                                </button>
                            </div>
                        </div>
                        <textarea
                            bind:value={step.description}
                            placeholder="Describe what to do..."
                            class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none resize-none"
                            rows="2"
                        ></textarea>
                        <div class="flex items-center gap-2">
                            <Clock size={14} class="text-foreground-subtle" />
                            <input
                                type="number"
                                bind:value={step.duration_min}
                                placeholder="Duration (min)"
                                class="w-32 bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none"
                            />
                        </div>
                    </div>
                {/each}
                {#if recipeSteps.length === 0}
                    <div
                        class="text-center py-6 border-2 border-dashed border-line rounded-xl text-foreground-subtle text-sm"
                    >
                        No steps added yet.
                    </div>
                {/if}
            </div>
        </section>
    </div>
</div>

<QuickAddIngredient bind:open={quickAddOpen} onCreated={onIngredientCreated} />

<style>
    /* Prevent number input arrows */
    input::-webkit-outer-spin-button,
    input::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }
    input[type="number"] {
        -moz-appearance: textfield;
    }
</style>
