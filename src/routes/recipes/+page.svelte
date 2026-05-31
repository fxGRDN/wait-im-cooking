<script lang="ts">
    import { onMount } from "svelte";
    import { getRecipes, getTags } from "$lib/services/recipes";
    import { checkAvailability } from "$lib/services/cooklog";
    import type {
        Recipe,
        Tag,
        RecipeWithTree,
        AvailabilityResult,
    } from "$lib/types";
    import { Search, Filter, X, History, CheckCircle2 } from "lucide-svelte";
    import { settings } from "$lib/stores/settings";
    import RecipeCard from "$lib/components/RecipeCard.svelte";

    let recipes: Recipe[] = $state([]);
    let tags: Tag[] = $state([]);
    let availabilityMap = $state<Record<string, boolean>>({});
    let loading = $state(true);

    let searchQuery = $state("");
    let selectedTagIds = $state<string[]>([]);
    let showFilters = $state(false);
    let filterCookable = $state(false);

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

            const isCookable = !filterCookable || availabilityMap[recipe.id];

            const matchesTags =
                selectedTagIds.length === 0 ||
                selectedTagIds.every((id) =>
                    recipe.tags.some((t) => t.id === id),
                );

            return matchesSearch && isCookable && matchesTags;
        }),
    );

    onMount(async () => {
        try {
            [recipes, tags] = await Promise.all([getRecipes(), getTags()]);

            // Fetch availability for each recipe in parallel
            const availabilities = await Promise.all(
                recipes.map((r) => checkAvailability(r.id)),
            );

            availabilities.forEach((a) => {
                availabilityMap[a.recipe_id] = a.cookable;
            });
        } catch (e) {
            console.error(e);
        } finally {
            loading = false;
        }
    });

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
                <a
                    href="/recipes/history"
                    class="flex items-center gap-2 px-3 py-2 rounded-xl bg-surface-sunken border border-line hover:bg-surface-raised transition text-foreground-muted hover:text-accent group"
                    title="View Cooking Log"
                >
                    <History
                        size={18}
                        class="group-hover:scale-110 transition-transform"
                    />
                    <span class="text-xs font-bold uppercase tracking-wider"
                        >Log</span
                    >
                </a>
                <button
                    onclick={() => (showFilters = !showFilters)}
                    class="p-2 rounded-xl border border-line hover:bg-surface-raised transition {selectedTagIds.length >
                        0 || filterCookable
                        ? 'bg-accent/10 border-accent text-accent'
                        : 'text-foreground-muted'}"
                >
                    <Filter size={18} />
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
            <div
                class="pt-2 animate-in slide-in-from-top-2 duration-200 space-y-4"
            >
                <div class="flex flex-col gap-2">
                    <p
                        class="text-xs font-bold text-foreground-subtle uppercase tracking-widest"
                    >
                        Availability
                    </p>
                    <div class="flex flex-wrap gap-2">
                        <button
                            onclick={() => (filterCookable = !filterCookable)}
                            class="px-3 py-1.5 rounded-full text-xs font-bold border flex items-center gap-1.5 transition {filterCookable
                                ? 'bg-success text-background border-success'
                                : 'bg-surface text-foreground-muted border-line hover:border-success/50'}"
                        >
                            <CheckCircle2 size={14} />
                            Cookable Only
                        </button>
                    </div>
                </div>

                <div class="flex flex-col gap-2">
                    <p
                        class="text-xs font-bold text-foreground-subtle uppercase tracking-widest"
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
                {#if searchQuery || selectedTagIds.length > 0 || filterCookable}
                    <button
                        onclick={() => {
                            searchQuery = "";
                            selectedTagIds = [];
                            filterCookable = false;
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
                    <RecipeCard
                        {recipe}
                        isCookable={!!availabilityMap[recipe.id]}
                    />
                {/each}
            </div>
        {/if}
    </div>
</div>
