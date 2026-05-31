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
        ChevronLeft,
        Save,
        Clock,
        ChefHat,
        Info,
        Camera,
        X as XIcon,
        Loader2,
    } from "@lucide/svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { saveImages } from "$lib/utils";
    import RecipeIngredientsEditor from "$lib/components/RecipeIngredientsEditor.svelte";
    import RecipeStepsEditor from "$lib/components/RecipeStepsEditor.svelte";
    import RecipeTagsEditor from "$lib/components/RecipeTagsEditor.svelte";
    import RecipeComponentsEditor from "$lib/components/RecipeComponentsEditor.svelte";

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

    let saving = $state(false);
    let error = $state<string | null>(null);

    onMount(async () => {
        try {
            [availableIngredients, availableRecipes, availableTags] =
                await Promise.all([getIngredients(), getRecipes(), getTags()]);
        } catch (e) {
            console.error(e);
        }
    });

    const onIngredientCreated = (newIng: Ingredient) => {
        availableIngredients = [...availableIngredients, newIng].sort((a, b) =>
            a.name.localeCompare(b.name),
        );
    };

    const handleCreateTag = async (name: string) => {
        try {
            const tag = await createTag(name);
            availableTags = [...availableTags, tag];
            tagIds = [...tagIds, tag.id];
        } catch (e) {
            console.error(e);
        }
    };

    const handleSave = async () => {
        if (!title.trim()) return;
        saving = true;
        error = null;

        try {
            const filteredIngredients = recipeIngredients.filter(
                (i) => i.ingredient_id,
            );
            const filteredComponents = recipeComponents.filter(
                (c) => c.child_id,
            );
            const filteredSteps = recipeSteps.filter((s) =>
                s.description.trim(),
            );

            const recipeId = await createRecipe(
                {
                    title,
                    description,
                    servings,
                    prep_time: prepTime,
                    cook_time: cookTime,
                    is_favourite: isFavourite,
                    cover_image: coverImage,
                },
                filteredIngredients,
                filteredComponents,
                filteredSteps,
                tagIds,
            );
            goto(`/recipes/${recipeId}`);
        } catch (e) {
            console.error(e);
            error = e instanceof Error ? e.message : "Failed to create recipe";
        } finally {
            saving = false;
        }
    };

    async function pickCover() {
        const savedPath = await saveImages("recipe_covers");
        if (savedPath && savedPath.length > 0) {
            coverImage = savedPath[0];
        }
    }
</script>

<div class="min-h-screen bg-surface text-foreground pb-20">
    <div
        class="border-b border-line px-4 pt-[calc(1rem+env(safe-area-inset-top))] pb-4 mb-4 sticky top-0 z-10 flex justify-between items-center bg-surface shadow-sm"
    >
        <div class="flex items-center gap-2">
            <button
                onclick={() => history.back()}
                class="p-2 -ml-2 hover:bg-surface-sunken rounded-full transition"
            >
                <ChevronLeft size={24} />
            </button>
            <h1 class="text-xl font-bold">New Recipe</h1>
        </div>
        <button
            onclick={handleSave}
            disabled={saving || !title.trim()}
            class="flex items-center gap-2 bg-accent text-background px-4 py-2 rounded-lg font-bold disabled:opacity-50 hover:opacity-90 transition shadow-lg shadow-accent/20"
        >
            {#if saving}
                <Loader2 size={18} class="animate-spin" />
            {:else}
                <Save size={18} />
            {/if}
            Save
        </button>
    </div>

    <main
        class="px-4 max-w-2xl mx-auto space-y-8 animate-in fade-in duration-500"
    >
        {#if error}
            <div
                class="p-4 bg-danger/10 text-danger rounded-xl border border-danger/20 flex items-center gap-2 text-sm font-bold"
            >
                <Info size={18} />
                {error}
            </div>
        {/if}

        <!-- Cover Image -->
        <section class="space-y-4">
            <div
                class="flex items-center gap-2 text-foreground-muted font-bold text-sm uppercase tracking-wider"
            >
                <Camera size={16} />
                <span>Cover Image</span>
            </div>
            {#if coverImage}
                <div
                    class="relative aspect-video rounded-2xl overflow-hidden border border-line shadow-sm group"
                >
                    <img
                        src={convertFileSrc(coverImage)}
                        alt="Cover"
                        class="w-full h-full object-cover"
                    />
                    <div
                        class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center gap-2"
                    >
                        <button
                            onclick={pickCover}
                            class="p-3 bg-white/20 backdrop-blur-md rounded-full text-white hover:bg-white/40 transition"
                        >
                            <Camera size={24} />
                        </button>
                        <button
                            onclick={() => (coverImage = null)}
                            class="p-3 bg-danger/20 backdrop-blur-md rounded-full text-danger-strong hover:bg-danger/40 transition"
                        >
                            <XIcon size={24} />
                        </button>
                    </div>
                </div>
            {:else}
                <button
                    onclick={pickCover}
                    class="w-full aspect-video rounded-2xl border-2 border-dashed border-line hover:border-accent/50 hover:bg-accent/5 transition-all flex flex-col items-center justify-center gap-2 text-foreground-subtle"
                >
                    <Camera size={32} strokeWidth={1.5} />
                    <span class="text-sm font-bold uppercase tracking-widest"
                        >Add Cover Photo</span
                    >
                </button>
            {/if}
        </section>

        <!-- Basic Info -->
        <section class="space-y-4">
            <div
                class="bg-surface p-6 rounded-3xl border border-line shadow-sm space-y-6"
            >
                <div class="space-y-2">
                    <label
                        class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest"
                        for="title">Recipe Title</label
                    >
                    <input
                        id="title"
                        type="text"
                        bind:value={title}
                        placeholder="e.g. Grandma's Secret Pasta"
                        class="w-full bg-transparent border-none p-0 text-2xl font-bold placeholder:text-foreground-muted/30 focus:ring-0"
                    />
                </div>

                <div class="space-y-2">
                    <label
                        class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest"
                        for="desc">Description</label
                    >
                    <textarea
                        id="desc"
                        bind:value={description}
                        placeholder="Tell a story about this dish..."
                        rows="2"
                        class="w-full bg-transparent border-none p-0 text-sm placeholder:text-foreground-muted/30 focus:ring-0 resize-none"
                    ></textarea>
                </div>

                <div class="grid grid-cols-3 gap-4 pt-4 border-t border-line">
                    <div class="space-y-1">
                        <label
                            class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1"
                        >
                            <ChefHat size={12} /> Servings
                        </label>
                        <input
                            type="number"
                            bind:value={servings}
                            placeholder="0"
                            class="w-full bg-surface-sunken border border-line rounded-xl px-3 py-2 text-sm font-bold outline-none focus:ring-1 focus:ring-accent/20"
                        />
                    </div>
                    <div class="space-y-1">
                        <label
                            class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1"
                        >
                            <Clock size={12} /> Prep (m)
                        </label>
                        <input
                            type="number"
                            bind:value={prepTime}
                            placeholder="0"
                            class="w-full bg-surface-sunken border border-line rounded-xl px-3 py-2 text-sm font-bold outline-none focus:ring-1 focus:ring-accent/20"
                        />
                    </div>
                    <div class="space-y-1">
                        <label
                            class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1"
                        >
                            <Clock size={12} /> Cook (m)
                        </label>
                        <input
                            type="number"
                            bind:value={cookTime}
                            placeholder="0"
                            class="w-full bg-surface-sunken border border-line rounded-xl px-3 py-2 text-sm font-bold outline-none focus:ring-1 focus:ring-accent/20"
                        />
                    </div>
                </div>

                <label
                    class="flex items-center gap-3 cursor-pointer select-none group"
                >
                    <div class="relative">
                        <input
                            type="checkbox"
                            bind:checked={isFavourite}
                            class="peer sr-only"
                        />
                        <div
                            class="w-10 h-6 bg-surface-sunken border border-line rounded-full peer-checked:bg-danger/10 peer-checked:border-danger transition-colors"
                        ></div>
                        <div
                            class="absolute left-1 top-1 w-4 h-4 bg-foreground-subtle rounded-full transition-all peer-checked:translate-x-4 peer-checked:bg-danger"
                        ></div>
                    </div>
                    <span
                        class="text-sm font-bold text-foreground-muted group-hover:text-foreground transition-colors"
                        >Mark as favorite</span
                    >
                </label>
            </div>
        </section>

        <!-- Tags Editor -->
        <RecipeTagsEditor
            bind:selectedTagIds={tagIds}
            {availableTags}
            onCreateTag={handleCreateTag}
        />

        <!-- Ingredients Editor -->
        <RecipeIngredientsEditor
            bind:ingredients={recipeIngredients}
            {availableIngredients}
            {onIngredientCreated}
        />

        <!-- Components Editor -->
        <RecipeComponentsEditor
            bind:components={recipeComponents}
            {availableRecipes}
        />

        <!-- Steps Editor -->
        <RecipeStepsEditor bind:steps={recipeSteps} />
    </main>
</div>
