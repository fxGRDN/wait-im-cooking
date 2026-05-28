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
    } from "lucide-svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";

    let logs = $state<RecipeHistoryWithImages[]>([]);
    let recipes = $state<Recipe[]>([]);
    let loading = $state(true);

    onMount(async () => {
        try {
            const [historyLogs, allRecipes] = await Promise.all([
                getCookLogs(), // Need to check if this returns images, or if I need getCookLog for each
                getRecipes(),
            ]);

            // For simplicity in the list, we'll fetch full details for logs that have images
            // In a real app, we might want to optimize this
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
    });

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
</script>

<div class="min-h-screen bg-surface text-foreground pb-20">
    <!-- Header -->
    <div
        class="border-b border-line px-4 py-4 mb-4 sticky top-0 z-10 flex items-center bg-surface"
    >
        <button
            onclick={() => history.back()}
            class="p-2 -ml-2 hover:bg-gray-100 rounded-full transition"
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
                        class="h-64 bg-gray-100 rounded-3xl animate-pulse"
                    ></div>
                {/each}
            </div>
        {:else if logs.length === 0}
            <div class="text-center py-20 text-gray-500">
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
                        class="bg-white rounded-3xl border border-line shadow-sm overflow-hidden flex flex-col"
                    >
                        <!-- Carousel -->
                        {#if log.images && log.images.length > 0}
                            <div class="relative group">
                                <div
                                    class="flex overflow-x-auto snap-x snap-mandatory no-scrollbar"
                                >
                                    {#each log.images as img}
                                        <div
                                            class="flex-shrink-0 w-full aspect-video snap-center"
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
                                                class="w-1.5 h-1.5 rounded-full bg-white/50"
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
                                class="aspect-video bg-gray-50 flex flex-col items-center justify-center text-gray-300 gap-2 border-b border-line"
                            >
                                <ImageIcon size={48} strokeWidth={1} />
                                <span class="text-xs font-medium"
                                    >No photos for this cook</span
                                >
                            </div>
                        {/if}

                        <div class="p-6 space-y-4">
                            <div class="flex justify-between items-start">
                                <div>
                                    <h3
                                        class="text-lg font-bold text-gray-900 leading-tight"
                                    >
                                        {getRecipeTitle(log.recipe_id)}
                                    </h3>
                                    <div
                                        class="flex items-center gap-2 mt-1 text-xs text-gray-500 font-medium"
                                    >
                                        <Calendar size={12} />
                                        {formatDate(log.created_at)}
                                    </div>
                                </div>
                                {#if log.rating}
                                    <div
                                        class="flex items-center gap-1 bg-amber-50 text-amber-600 px-2 py-1 rounded-lg"
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
                                    class="text-sm text-gray-600 leading-relaxed italic bg-gray-50 p-4 rounded-2xl border border-gray-100"
                                >
                                    "{log.notes}"
                                </p>
                            {/if}

                            <div class="flex items-center gap-4 pt-2">
                                {#if log.servings_made}
                                    <div
                                        class="flex items-center gap-1.5 text-xs font-bold text-gray-400"
                                    >
                                        <ChefHat size={14} />
                                        {log.servings_made} SERVINGS
                                    </div>
                                {/if}
                                {#if log.duration_min}
                                    <div
                                        class="flex items-center gap-1.5 text-xs font-bold text-gray-400"
                                    >
                                        <Clock size={14} />
                                        {log.duration_min} MIN
                                    </div>
                                {/if}
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
