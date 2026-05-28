<script lang="ts">
    import { onMount } from "svelte";
    import { getRecipes } from "$lib/services/recipes";
    import type { Recipe } from "$lib/types";
    import { Clock, ChefHat, Heart } from "lucide-svelte";

    let recipes: Recipe[] = $state([]);
    let loading = $state(true);

    onMount(async () => {
        try {
            recipes = await getRecipes();
        } catch (e) {
            console.error(e);
        } finally {
            loading = false;
        }
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
        class="border-b border-line px-4 py-4 mb-4 sticky top-0 z-10 flex justify-between items-center bg-surface"
    >
        <h1 class="text-2xl font-bold">Recipes</h1>
    </div>

    <a
        href="/recipes/add"
        class="fixed bottom-24 right-6 w-14 h-14 bg-accent text-background rounded-full flex items-center justify-center shadow-lg hover:opacity-90 transition z-40"
        aria-label="Add Recipe"
    >
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="28"
            height="28"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2.5"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <line x1="12" y1="5" x2="12" y2="19"></line>
            <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
    </a>

    <div class="px-4 max-w-2xl mx-auto">
        {#if loading}
            <div class="space-y-4">
                {#each Array(3) as _}
                    <div
                        class="animate-pulse bg-white rounded-xl h-24 border border-line"
                    ></div>
                {/each}
            </div>
        {:else if recipes.length === 0}
            <div class="text-center py-12 text-gray-500">
                <p>No recipes found.</p>
                <p class="mt-2 text-sm">
                    Create your first recipe to get started!
                </p>
            </div>
        {:else}
            <div class="grid grid-cols-1 gap-4">
                {#each recipes as recipe}
                    <a
                        href="/recipes/{recipe.id}"
                        class="bg-surface rounded-xl shadow-sm border border-line overflow-hidden hover:border-accent/50 transition flex group"
                    >
                        {#if recipe.cover_image}
                            <div
                                class="w-24 h-24 sm:w-32 sm:h-32 flex-shrink-0"
                            >
                                <img
                                    src={recipe.cover_image}
                                    alt={recipe.title}
                                    class="w-full h-full object-cover"
                                />
                            </div>
                        {:else}
                            <div
                                class="w-24 h-24 sm:w-32 sm:h-32 flex-shrink-0 bg-gray-100 flex items-center justify-center text-gray-400"
                            >
                                <ChefHat size={32} />
                            </div>
                        {/if}

                        <div class="p-4 flex-1 flex flex-col justify-between">
                            <div>
                                <div class="flex justify-between items-start">
                                    <h2
                                        class="text-lg font-bold group-hover:text-accent transition line-clamp-1"
                                    >
                                        {recipe.title}
                                    </h2>
                                    {#if recipe.is_favourite}
                                        <Heart
                                            size={16}
                                            class="text-red-500 fill-current"
                                        />
                                    {/if}
                                </div>
                                {#if recipe.description}
                                    <p
                                        class="text-sm text-gray-600 line-clamp-1 mt-1"
                                    >
                                        {recipe.description}
                                    </p>
                                {/if}
                            </div>

                            <div
                                class="flex items-center gap-4 mt-2 text-xs text-gray-500 font-medium"
                            >
                                {#if recipe.prep_time}
                                    <span class="flex items-center gap-1">
                                        <Clock size={12} />
                                        Prep: {formatTime(recipe.prep_time)}
                                    </span>
                                {/if}
                                {#if recipe.cook_time}
                                    <span class="flex items-center gap-1">
                                        <ChefHat size={12} />
                                        Cook: {formatTime(recipe.cook_time)}
                                    </span>
                                {/if}
                            </div>
                        </div>
                    </a>
                {/each}
            </div>
        {/if}
    </div>
</div>
