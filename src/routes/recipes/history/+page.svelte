<script lang="ts">
    import { onMount } from "svelte";
    import { getCookLogs } from "$lib/services/cooklog";
    import { getRecipes } from "$lib/services/recipes";
    import type { RecipeHistoryWithImages, Recipe } from "$lib/types";
    import {
        ChevronLeft,
        Star,
        Calendar,
        Clock,
        Image as ImageIcon,
        ChefHat,
        Edit2,
        ExternalLink,
    } from "lucide-svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { goto } from "$app/navigation";

    let logs = $state<RecipeHistoryWithImages[]>([]);
    let recipes = $state<Recipe[]>([]);
    let loading = $state(true);

    async function loadData() {
        loading = true;
        try {
            const [historyLogs, allRecipes] = await Promise.all([
                getCookLogs(),
                getRecipes(),
            ]);

            const logsWithDetails = await Promise.all(
                historyLogs.map(async (log) => {
                    const detail = await (
                        await import("$lib/services/cooklog")
                    ).getCookLog(log.id);
                    return detail || { ...log, images: [] };
                }),
            );

            logs = logsWithDetails;
            recipes = allRecipes;
        } catch (e) {
            console.error(e);
        } finally {
            loading = false;
        }
    }

    onMount(loadData);

    function getRecipeTitle(id: string) {
        return recipes.find((r) => r.id === id)?.title || "Unknown Recipe";
    }

    function formatDate(dateStr: string) {
        return new Date(dateStr).toLocaleDateString(undefined, {
            weekday: "short",
            year: "numeric",
            month: "long",
            day: "numeric",
        });
    }

    function goToRecipe(id: string) {
        goto(`/recipes/${id}`);
    }

    function openEditPage(id: string) {
        goto(`/recipes/history/${id}`);
    }
</script>

<div class="min-h-screen bg-surface text-foreground pb-20">
    <!-- Header -->
    <div
        class="border-b border-line px-4 pt-[calc(1rem+env(safe-area-inset-top))] pb-4 mb-4 sticky top-0 z-10 flex items-center bg-surface"
    >
        <button
            onclick={() => history.back()}
            class="p-2 -ml-2 hover:bg-surface-sunken rounded-full transition"
        >
            <ChevronLeft size={24} />
        </button>
        <h1 class="text-xl font-bold ml-2">Cooking History</h1>
    </div>

    <div class="px-4 max-w-2xl mx-auto space-y-6">
        {#if loading}
            <div class="space-y-6">
                {#each Array(3) as _}
                    <div
                        class="h-64 bg-surface-sunken rounded-3xl animate-pulse"
                    ></div>
                {/each}
            </div>
        {:else if logs.length === 0}
            <div class="text-center py-20 text-foreground-muted">
                <Calendar size={48} class="mx-auto mb-4 opacity-20" />
                <p>No cooking history yet.</p>
                <a
                    href="/recipes"
                    class="text-accent font-bold mt-2 inline-block"
                    >Start cooking!</a
                >
            </div>
        {:else}
            <div class="space-y-8">
                {#each logs as log}
                    <div
                        class="bg-surface rounded-3xl border border-line shadow-sm overflow-hidden flex flex-col"
                    >
                        <!-- Carousel -->
                        {#if log.images && log.images.length > 0}
                            <div class="relative group">
                                <div
                                    class="flex overflow-x-auto snap-x snap-mandatory no-scrollbar"
                                >
                                    {#each log.images as img}
                                        <div
                                            class="shrink-0 w-full aspect-video snap-center"
                                        >
                                            <img
                                                src={convertFileSrc(
                                                    img.file_path,
                                                )}
                                                alt="Dish"
                                                class="w-full h-full object-cover"
                                            />
                                        </div>
                                    {/each}
                                </div>
                                {#if log.images.length > 1}
                                    <div
                                        class="absolute bottom-4 left-1/2 -translate-x-1/2 flex gap-1.5 px-3 py-1.5 bg-black/20 backdrop-blur-md rounded-full"
                                    >
                                        {#each log.images as _, i}
                                            <div
                                                class="w-1.5 h-1.5 rounded-full bg-surface/50"
                                            ></div>
                                        {/each}
                                    </div>
                                    <div
                                        class="absolute top-4 right-4 bg-black/40 text-white text-[10px] font-bold px-2 py-1 rounded-md backdrop-blur-md"
                                    >
                                        {log.images.length} PHOTOS
                                    </div>
                                {/if}
                            </div>
                        {:else}
                            <div
                                class="aspect-video bg-surface-sunken flex flex-col items-center justify-center text-foreground-subtle gap-2 border-b border-line"
                            >
                                <ImageIcon size={48} strokeWidth={1} />
                                <span class="text-xs font-medium"
                                    >No photos for this cook</span
                                >
                            </div>
                        {/if}

                        <div class="p-6 space-y-4">
                            <div class="flex justify-between items-start">
                                <div class="flex-1 min-w-0">
                                    <h3
                                        class="text-lg font-bold text-foreground leading-tight truncate"
                                    >
                                        {getRecipeTitle(log.recipe_id)}
                                    </h3>
                                    <div
                                        class="flex items-center gap-2 mt-1 text-xs text-foreground-muted font-medium"
                                    >
                                        <Calendar size={12} />
                                        {formatDate(log.created_at)}
                                    </div>
                                </div>
                                {#if log.rating}
                                    <div
                                        class="flex items-center gap-1 bg-secondary-soft text-secondary px-2 py-1 rounded-lg ml-4"
                                    >
                                        <Star size={14} fill="currentColor" />
                                        <span class="font-bold text-sm"
                                            >{log.rating}</span
                                        >
                                    </div>
                                {/if}
                            </div>

                            {#if log.notes}
                                <p
                                    class="text-sm text-foreground-muted leading-relaxed italic bg-surface-sunken p-4 rounded-2xl border border-line"
                                >
                                    "{log.notes}"
                                </p>
                            {/if}

                            <div class="flex items-center justify-between pt-2">
                                <div class="flex items-center gap-4">
                                    {#if log.servings_made}
                                        <div
                                            class="flex items-center gap-1.5 text-xs font-bold text-foreground-subtle"
                                        >
                                            <ChefHat size={14} />
                                            {log.servings_made} SERVINGS
                                        </div>
                                    {/if}
                                    {#if log.duration_min}
                                        <div
                                            class="flex items-center gap-1.5 text-xs font-bold text-foreground-subtle"
                                        >
                                            <Clock size={14} />
                                            {log.duration_min} MIN
                                        </div>
                                    {/if}
                                </div>

                                <div class="flex items-center gap-2">
                                    <button
                                        onclick={() =>
                                            goToRecipe(log.recipe_id)}
                                        class="p-2 text-foreground-subtle hover:text-accent hover:bg-accent/5 rounded-full transition"
                                        title="Go to Recipe"
                                    >
                                        <ExternalLink size={18} />
                                    </button>
                                    <button
                                        onclick={() => openEditPage(log.id)}
                                        class="p-2 text-foreground-subtle hover:text-accent hover:bg-accent/5 rounded-full transition"
                                        title="Edit Cook"
                                    >
                                        <Edit2 size={18} />
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                {/each}
            </div>
        {/if}
    </div>
</div>

<style>
    .no-scrollbar::-webkit-scrollbar {
        display: none;
    }
    .no-scrollbar {
        -ms-overflow-style: none;
        scrollbar-width: none;
    }
</style>
