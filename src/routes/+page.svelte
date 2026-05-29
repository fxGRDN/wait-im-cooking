<script lang="ts">
    import { onMount } from "svelte";
    import { getCookLogs } from "$lib/services/cooklog";
    import { getInventory } from "$lib/services/ingredients";
    import { getRecipes } from "$lib/services/recipes";
    import type {
        RecipeHistory,
        IngredientWithInventory,
        Recipe,
    } from "$lib/types";
    import {
        Clock,
        ChevronRight,
        AlertTriangle,
        ChefHat,
        History,
        TrendingUp,
    } from "lucide-svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";

    let recentLogs = $state<RecipeHistory[]>([]);
    let lowIngredients = $state<IngredientWithInventory[]>([]);
    let recipes = $state<Recipe[]>([]);
    let loading = $state(true);

    onMount(async () => {
        try {
            const [logs, inventory, allRecipes] = await Promise.all([
                getCookLogs(),
                getInventory(),
                getRecipes(),
            ]);

            // Take 5 most recent logs
            recentLogs = logs.slice(0, 5);

            // Filter ingredients with low quantity based on their restock threshold
            lowIngredients = inventory
                .filter((i) => {
                    if (!i.inventory) return false;
                    // If threshold is set, use it. Otherwise use a default of 2.
                    const threshold = i.restock_threshold ?? 2;
                    return i.inventory.quantity <= threshold;
                })
                .slice(0, 5);

            recipes = allRecipes;
        } catch (e) {
            console.error(e);
        } finally {
            loading = false;
        }
    });

    function getRecipe(id: string) {
        return recipes.find((r) => r.id === id);
    }

    function getRecipeTitle(id: string) {
        return getRecipe(id)?.title || "Unknown Recipe";
    }

    function getRecipeImage(id: string) {
        const r = getRecipe(id);
        return r?.cover_image ? convertFileSrc(r.cover_image) : null;
    }

    function formatDate(dateStr: string) {
        const date = new Date(dateStr);
        return date.toLocaleDateString(undefined, {
            month: "short",
            day: "numeric",
        });
    }
</script>

<div class="min-h-screen bg-surface text-foreground pb-20">
    <!-- Header -->
    <div
        class="px-4 pt-[calc(1.5rem+env(safe-area-inset-top))] pb-6 border-b border-line bg-surface sticky top-0 z-10"
    >
        <div class="flex items-center gap-3">
            <div
                class="w-10 h-10 bg-accent rounded-xl flex items-center justify-center text-background shadow-lg shadow-accent/20"
            >
                <ChefHat size={24} />
            </div>
            <div>
                <h1 class="text-xl font-bold leading-none">
                    Wait, I'm Cooking!
                </h1>
                <p
                    class="text-xs text-foreground-muted mt-1 font-medium uppercase tracking-wider"
                >
                    Kitchen Dashboard
                </p>
            </div>
        </div>
    </div>

    <div class="px-4 max-w-2xl mx-auto space-y-8 mt-6">
        {#if loading}
            <div class="space-y-6">
                <div
                    class="h-40 bg-surface-sunken rounded-2xl animate-pulse"
                ></div>
                <div
                    class="h-64 bg-surface-sunken rounded-2xl animate-pulse"
                ></div>
            </div>
        {:else}
            <!-- Running Low Section -->
            <section class="space-y-4">
                <div class="flex items-center justify-between">
                    <h2
                        class="text-sm font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-2"
                    >
                        <AlertTriangle size={16} class="text-secondary" />
                        Running Low
                    </h2>
                    <a
                        href="/ingredients"
                        class="text-xs font-bold text-accent hover:underline"
                        >View Pantry</a
                    >
                </div>

                {#if lowIngredients.length === 0}
                    <div
                        class="bg-surface p-6 rounded-2xl border border-line text-center"
                    >
                        <p class="text-sm text-foreground-muted">
                            Pantry is well stocked!
                        </p>
                    </div>
                {:else}
                    <div
                        class="bg-surface rounded-2xl border border-line shadow-sm overflow-hidden"
                    >
                        <ul class="divide-y divide-line">
                            {#each lowIngredients as item}
                                <li
                                    class="p-4 flex items-center justify-between hover:bg-surface-raised transition"
                                >
                                    <div class="flex flex-col">
                                        <span class="font-bold"
                                            >{item.name}</span
                                        >
                                        <span
                                            class="text-xs text-secondary font-medium"
                                            >Only {item.inventory?.quantity}
                                            {item.inventory?.unit} left</span
                                        >
                                    </div>
                                    <a
                                        href="/ingredients"
                                        class="p-2 text-foreground-subtle hover:text-accent transition"
                                    >
                                        <ChevronRight size={20} />
                                    </a>
                                </li>
                            {/each}
                        </ul>
                    </div>
                {/if}
            </section>

            <!-- Recently Made Section -->
            <section class="space-y-4">
                <div class="flex items-center justify-between">
                    <h2
                        class="text-sm font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-2"
                    >
                        <History size={16} class="text-accent" />
                        Recently Made
                    </h2>
                    <a
                        href="/recipes/history"
                        class="text-xs font-bold text-accent hover:underline"
                        >Full History</a
                    >
                </div>

                {#if recentLogs.length === 0}
                    <div
                        class="bg-surface p-10 rounded-2xl border border-dashed border-line text-center space-y-4"
                    >
                        <div
                            class="w-12 h-12 bg-surface-sunken rounded-full flex items-center justify-center mx-auto text-line-strong"
                        >
                            <TrendingUp size={24} />
                        </div>
                        <div>
                            <p
                                class="text-sm font-medium text-foreground-muted"
                            >
                                No cooking history yet
                            </p>
                            <p class="text-xs text-foreground-subtle mt-1">
                                Your recent cooks will appear here.
                            </p>
                        </div>
                        <a
                            href="/recipes"
                            class="inline-block bg-accent text-background px-4 py-2 rounded-lg text-sm font-bold shadow-lg shadow-accent/20"
                        >
                            Find a Recipe
                        </a>
                    </div>
                {:else}
                    <div class="grid gap-3">
                        {#each recentLogs as log}
                            <a
                                href="/recipes/history/{log.id}"
                                class="bg-surface p-4 rounded-2xl border border-line shadow-sm hover:border-accent/50 transition flex items-center gap-4 group"
                            >
                                <div class="flex-1 min-w-0">
                                    <h3
                                        class="font-bold group-hover:text-accent transition truncate"
                                    >
                                        {getRecipeTitle(log.recipe_id)}
                                    </h3>
                                    <div
                                        class="flex items-center gap-3 mt-1 text-xs text-foreground-muted font-medium"
                                    >
                                        <span class="flex items-center gap-1">
                                            <Clock size={12} />
                                            {formatDate(log.created_at)}
                                        </span>
                                        {#if log.rating}
                                            <span
                                                class="flex items-center gap-0.5 text-secondary"
                                            >
                                                ★ {log.rating}
                                            </span>
                                        {/if}
                                    </div>
                                </div>
                                <ChevronRight
                                    size={18}
                                    class="text-line-strong group-hover:text-accent transition"
                                />
                            </a>
                        {/each}
                    </div>
                {/if}
            </section>
        {/if}
    </div>
</div>
