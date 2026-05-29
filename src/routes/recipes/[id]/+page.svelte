<script lang="ts">
    import { onMount } from "svelte";
    import { page } from "$app/stores";
    import {
        getRecipeWithTree,
        deleteRecipe,
        toggleFavourite,
    } from "$lib/services/recipes";
    import { goto } from "$app/navigation";
    import type { RecipeWithTree } from "$lib/types";
    import {
        ChevronLeft,
        Clock,
        ChefHat,
        Users,
        Heart,
        Trash2,
        Edit,
        CheckCircle2,
        Circle,
        Play,
    } from "lucide-svelte";

    import { convertFileSrc } from "@tauri-apps/api/core";
    import { settings } from "$lib/stores/settings";

    let recipe = $state<RecipeWithTree | null>(null);
    let loading = $state(true);
    let error = $state<string | null>(null);

    const id = $page.params.id;

    onMount(async () => {
        try {
            recipe = await getRecipeWithTree(id);
            if (!recipe) error = "Recipe not found";
        } catch (e) {
            console.error(e);
            error = "Failed to load recipe";
        } finally {
            loading = false;
        }
    });

    const handleToggleFav = async () => {
        if (!recipe) return;
        try {
            await toggleFavourite(recipe.id);
            recipe.is_favourite = !recipe.is_favourite;
        } catch (e) {
            console.error(e);
        }
    };

    const handleDelete = async () => {
        if (!recipe || !confirm("Are you sure you want to delete this recipe?"))
            return;
        try {
            await deleteRecipe(recipe.id);
            goto("/recipes");
        } catch (e) {
            console.error(e);
            alert("Failed to delete recipe");
        }
    };

    const startCooking = () => {
        goto(`/recipes/${id}/cook`);
    };

    onMount(() => {
        window.addEventListener("start-cooking", startCooking);
        return () => window.removeEventListener("start-cooking", startCooking);
    });

    function formatTime(minutes: number | null) {
        if (!minutes) return null;
        if (minutes < 60) return `${minutes}m`;
        const h = Math.floor(minutes / 60);
        const m = minutes % 60;
        return m > 0 ? `${h}h ${m}m` : `${h}h`;
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
            <h1 class="text-xl font-bold truncate max-w-[200px]">
                {recipe?.title || "Recipe"}
            </h1>
        </div>
        <div class="flex items-center gap-1">
            <button
                onclick={handleToggleFav}
                class="p-2 hover:bg-surface-sunken rounded-full transition {recipe?.is_favourite
                    ? 'text-danger'
                    : 'text-foreground-subtle'}"
            >
                <Heart
                    size={20}
                    fill={recipe?.is_favourite ? "currentColor" : "none"}
                />
            </button>
            <button
                onclick={handleDelete}
                class="p-2 hover:bg-surface-sunken rounded-full transition text-foreground-subtle hover:text-danger"
            >
                <Trash2 size={20} />
            </button>
        </div>
    </div>

    {#if loading}
        <div class="px-4 max-w-2xl mx-auto space-y-4 animate-pulse">
            <div class="h-48 bg-surface-sunken rounded-2xl w-full"></div>
            <div class="h-8 bg-surface-sunken rounded w-3/4"></div>
            <div class="h-4 bg-surface-sunken rounded w-1/2"></div>
        </div>
    {:else if error}
        <div class="px-4 text-center py-12">
            <p class="text-danger">{error}</p>
            <a href="/recipes" class="text-accent font-bold mt-4 inline-block"
                >Back to recipes</a
            >
        </div>
    {:else if recipe}
        <div class="px-4 max-w-2xl mx-auto space-y-8">
            <!-- Header/Meta -->
            <section class="space-y-4">
                {#if recipe.cover_image}
                    <img
                        src={convertFileSrc(recipe.cover_image)}
                        alt={recipe.title}
                        class="w-full h-48 sm:h-64 object-cover rounded-2xl shadow-sm border border-line"
                    />
                {/if}

                <div class="space-y-2">
                    <h2 class="text-2xl font-bold">{recipe.title}</h2>
                    {#if recipe.description}
                        <p class="text-foreground-muted leading-relaxed">
                            {recipe.description}
                        </p>
                    {/if}
                </div>

                <div
                    class="flex flex-wrap gap-4 py-4 border-y border-line text-sm font-medium text-foreground-muted"
                >
                    {#if recipe.servings}
                        <div class="flex items-center gap-1.5">
                            <Users size={18} class="text-accent" />
                            <span>{recipe.servings} servings</span>
                        </div>
                    {/if}
                    {#if recipe.prep_time}
                        <div class="flex items-center gap-1.5">
                            <Clock size={18} class="text-accent" />
                            <span>Prep: {formatTime(recipe.prep_time)}</span>
                        </div>
                    {/if}
                    {#if recipe.cook_time}
                        <div class="flex items-center gap-1.5">
                            <ChefHat size={18} class="text-accent" />
                            <span>Cook: {formatTime(recipe.cook_time)}</span>
                        </div>
                    {/if}
                </div>

                {#if recipe.tags.length > 0}
                    <div class="flex flex-wrap gap-2">
                        {#each recipe.tags as tag}
                            <span
                                class="px-3 py-1 bg-accent/10 text-accent rounded-full text-xs font-bold"
                            >
                                {tag.name}
                            </span>
                        {/each}
                    </div>
                {/if}
            </section>

            <!-- Ingredients -->
            <section class="space-y-4">
                <h3 class="text-lg font-bold flex items-center gap-2">
                    <ChefHat size={20} class="text-accent" />
                    Ingredients
                </h3>
                <div
                    class="bg-surface rounded-2xl border border-line overflow-hidden shadow-sm"
                >
                    <ul class="divide-y divide-line">
                        {#each recipe.ingredients as ri}
                            <li
                                class="p-4 flex justify-between items-center hover:bg-surface-raised transition"
                            >
                                <div class="flex flex-col">
                                    <span
                                        class="font-medium {ri.is_optional
                                            ? 'text-foreground-subtle'
                                            : ''}"
                                    >
                                        {ri.ingredient.name}
                                        {#if ri.is_optional}
                                            <span
                                                class="text-xs font-normal ml-1"
                                                >(optional)</span
                                            >
                                        {/if}
                                    </span>
                                </div>
                                <span
                                    class="text-sm font-bold bg-surface-sunken px-2 py-1 rounded-lg"
                                >
                                    {ri.quantity}
                                    {ri.unit}
                                </span>
                            </li>
                        {/each}
                        {#each recipe.components as comp}
                            <li
                                class="p-4 flex justify-between items-center hover:bg-surface-raised transition"
                            >
                                <a
                                    href="/recipes/{comp.child.id}"
                                    class="font-medium text-accent hover:underline"
                                >
                                    {comp.child.title} (sub-recipe)
                                </a>
                                <span
                                    class="text-sm font-bold bg-accent/10 text-accent px-2 py-1 rounded-lg"
                                >
                                    {comp.servings_needed} servings
                                </span>
                            </li>
                        {/each}
                    </ul>
                </div>
            </section>

            <!-- Steps -->
            <section class="space-y-4 pb-12">
                <h3 class="text-lg font-bold flex items-center gap-2">
                    <Clock size={20} class="text-accent" />
                    Steps
                </h3>
                <div class="space-y-4">
                    {#each recipe.steps as step, i}
                        <div class="flex gap-4">
                            <div class="flex flex-col items-center gap-2">
                                <div
                                    class="w-8 h-8 rounded-full bg-accent text-background flex items-center justify-center font-bold flex-shrink-0"
                                >
                                    {i + 1}
                                </div>
                                {#if i < recipe.steps.length - 1}
                                    <div class="w-0.5 flex-1 bg-line"></div>
                                {/if}
                            </div>
                            <div class="flex-1 pb-6">
                                <div
                                    class="bg-surface p-4 rounded-2xl border border-line shadow-sm space-y-2"
                                >
                                    <div
                                        class="flex justify-between items-start"
                                    >
                                        <span
                                            class="text-xs font-bold uppercase tracking-wider text-foreground-subtle"
                                        >
                                            {step.step_type}
                                        </span>
                                        {#if step.duration_min}
                                            <span
                                                class="text-xs font-bold text-accent bg-accent/5 px-2 py-0.5 rounded-full"
                                            >
                                                {formatTime(step.duration_min)}
                                            </span>
                                        {/if}
                                    </div>
                                    <p
                                        class="text-foreground whitespace-pre-wrap"
                                    >
                                        {step.description}
                                    </p>
                                </div>
                            </div>
                        </div>
                    {/each}
                </div>
            </section>
        </div>
    {/if}
</div>
