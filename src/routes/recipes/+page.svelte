<script lang="ts">
    import { onMount } from "svelte";
    import { getRecipes, getTags } from "$lib/services/recipes";
    import type { Recipe, Tag, RecipeWithTree } from "$lib/types";
    import {
        Clock,
        ChefHat,
        Heart,
        Search,
        Filter,
        X,
        Plus,
    } from "lucide-svelte";
    import { invoke, convertFileSrc } from "@tauri-apps/api/core";
    import { settings } from "$lib/stores/settings";

    let recipes: Recipe[] = $state([]);
    let tags: Tag[] = $state([]);
    let loading = $state(true);

    let searchQuery = $state("");
    let selectedTagIds = $state<string[]>([]);
    let showFilters = $state(false);

    // We need to fetch full recipe trees to filter by tags accurately,
    // or the backend needs to provide tags in the basic get_recipes call.
    // For now, let's assume we might need to fetch trees or adjust the backend.
    // Actually, get_recipes in backend returns Vec<Recipe>, which doesn't have tags.
    // Let's implement a more efficient way if possible, but for now, we'll fetch all.

    let filteredRecipes = $derived(
        recipes.filter((recipe) => {
            const matchesSearch =
                recipe.title
                    .toLowerCase()
                    .includes(searchQuery.toLowerCase()) ||
                (recipe.description
                    ?.toLowerCase()
                    .includes(searchQuery.toLowerCase()) ??
                    false);

            // Note: Basic Recipe type doesn't have tags.
            // To filter by tags properly, we'd need them in the list.
            // I will update the backend or fetch trees if needed, but for now search is priority.
            return matchesSearch;
        }),
    );

    onMount(async () => {
        try {
            [recipes, tags] = await Promise.all([getRecipes(), getTags()]);
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

    function toggleTag(tagId: string) {
        if (selectedTagIds.includes(tagId)) {
            selectedTagIds = selectedTagIds.filter((id) => id !== tagId);
        } else {
            selectedTagIds = [...selectedTagIds, tagId];
        }
    }
</script>

<div class="min-h-screen bg-surface text-foreground pb-20">
    <div
        class="border-b border-line px-4 pt-[calc(1rem+env(safe-area-inset-top))] pb-4 mb-4 sticky top-0 z-10 bg-surface space-y-4"
    >
        <div class="flex justify-between items-center">
            <h1 class="text-2xl font-bold">Recipes</h1>
            <div class="flex gap-2">
                <button
                    onclick={() => (showFilters = !showFilters)}
                    class="p-2 rounded-lg border border-line hover:bg-surface-raised transition {selectedTagIds.length >
                    0
                        ? 'bg-accent/10 border-accent text-accent'
                        : ''}"
                >
                    <Filter size={20} />
                </button>
            </div>
        </div>

        <!-- Search Bar -->
        <div class="relative">
            <Search
                class="absolute left-3 top-1/2 -translate-y-1/2 text-foreground-subtle"
                size={18}
            />
            <input
                type="text"
                bind:value={searchQuery}
                placeholder="Search recipes..."
                class="w-full pl-10 pr-10 py-2 bg-surface-sunken border border-line rounded-xl focus:ring-2 focus:ring-accent/20 focus:border-accent outline-none transition"
            />
            {#if searchQuery}
                <button
                    onclick={() => (searchQuery = "")}
                    class="absolute right-3 top-1/2 -translate-y-1/2 text-foreground-subtle hover:text-foreground-muted"
                >
                    <X size={18} />
                </button>
            {/if}
        </div>

        <!-- Filters Drawer/Section -->
        {#if showFilters}
            <div class="pt-2 animate-in slide-in-from-top-2 duration-200">
                <p
                    class="text-xs font-bold text-foreground-subtle uppercase tracking-widest mb-2"
                >
                    Filter by Tags
                </p>
                <div class="flex flex-wrap gap-2">
                    {#each tags as tag}
                        <button
                            onclick={() => toggleTag(tag.id)}
                            class="px-3 py-1.5 rounded-full text-xs font-medium border transition {selectedTagIds.includes(
                                tag.id,
                            )
                                ? 'bg-accent text-background border-accent'
                                : 'bg-surface text-foreground-muted border-line hover:border-accent/50'}"
                        >
                            {tag.name}
                        </button>
                    {/each}
                    {#if tags.length === 0}
                        <p class="text-xs text-foreground-subtle italic">
                            No tags found.
                        </p>
                    {/if}
                </div>
            </div>
        {/if}
    </div>

    <div class="px-4 max-w-2xl mx-auto">
        {#if loading}
            <div class="space-y-4">
                {#each Array(3) as _}
                    <div
                        class="animate-pulse bg-surface-sunken rounded-xl h-24 border border-line"
                    ></div>
                {/each}
            </div>
        {:else if filteredRecipes.length === 0}
            <div class="text-center py-12 text-foreground-muted">
                <Search size={48} class="mx-auto mb-4 opacity-20" />
                <p>No recipes match your criteria.</p>
                {#if searchQuery || selectedTagIds.length > 0}
                    <button
                        onclick={() => {
                            searchQuery = "";
                            selectedTagIds = [];
                        }}
                        class="mt-4 text-accent font-bold text-sm"
                    >
                        Clear all filters
                    </button>
                {/if}
            </div>
        {:else}
            <div class="grid grid-cols-1 gap-4">
                {#each filteredRecipes as recipe}
                    <a
                        href="/recipes/{recipe.id}"
                        class="bg-surface rounded-xl shadow-sm border border-line overflow-hidden hover:border-accent/50 transition flex group"
                    >
                        {#if recipe.cover_image}
                            <div
                                class="w-24 h-24 sm:w-32 sm:h-32 flex-shrink-0"
                            >
                                <img
                                    src={convertFileSrc(recipe.cover_image)}
                                    alt={recipe.title}
                                    class="w-full h-full object-cover"
                                />
                            </div>
                        {:else}
                            <div
                                class="w-24 h-24 sm:w-32 sm:h-32 flex-shrink-0 bg-surface-sunken flex items-center justify-center text-foreground-subtle"
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
                                            class="text-danger fill-current"
                                        />
                                    {/if}
                                </div>
                                {#if recipe.description}
                                    <p
                                        class="text-sm text-foreground-muted line-clamp-1 mt-1"
                                    >
                                        {recipe.description}
                                    </p>
                                {/if}
                            </div>

                            <div
                                class="flex items-center gap-4 mt-2 text-xs text-foreground-muted font-medium"
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
