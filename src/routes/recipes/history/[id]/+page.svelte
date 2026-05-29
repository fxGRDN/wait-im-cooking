<script lang="ts">
    import { onMount } from "svelte";
    import { page } from "$app/stores";
    import {
        getCookLog,
        updateCookLog,
        deleteCookLog,
    } from "$lib/services/cooklog";
    import { getRecipe } from "$lib/services/recipes";
    import type { RecipeHistoryWithImages, Recipe } from "$lib/types";
    import {
        ChevronLeft,
        Star,
        Calendar,
        Clock,
        Camera,
        X,
        Save,
        Trash2,
        ChefHat,
        CheckCircle2,
        Loader2,
        Edit2,
        RotateCcw,
        MapPin,
    } from "lucide-svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { remove } from "@tauri-apps/plugin-fs";
    import { saveImages } from "$lib/utils";
    import { goto } from "$app/navigation";

    const id = $page.params.id as string;

    // Check if we should start in edit mode (from cook completion)
    let isEditing = $state($page.url.searchParams.get("edit") === "true");

    let log = $state<RecipeHistoryWithImages | null>(null);
    let recipe = $state<Recipe | null>(null);
    let loading = $state(true);
    let saving = $state(false);

    // Form state (for editing)
    let rating = $state(5);
    let notes = $state("");
    let servings = $state(1);
    let duration = $state(0);
    let newImages = $state<string[]>([]);
    let imagesToRemove = $state<string[]>([]);

    onMount(async () => {
        try {
            log = await getCookLog(id);
            if (log) {
                recipe = await getRecipe(log.recipe_id);
                // Initialize form state
                rating = log.rating || 5;
                notes = log.notes || "";
                servings = log.servings_made || 1;
                duration = log.duration_min || 0;
            }
        } catch (e) {
            console.error(e);
        } finally {
            loading = false;
        }
    });

    async function pickImages() {
        const selected = await saveImages("history_images", true);
        newImages = [...newImages, ...selected];
    }

    function toggleRemoveImage(imgId: string) {
        if (imagesToRemove.includes(imgId)) {
            imagesToRemove = imagesToRemove.filter((i) => i !== imgId);
        } else {
            imagesToRemove = [...imagesToRemove, imgId];
        }
    }

    async function handleSave() {
        if (!log) return;
        saving = true;
        try {
            await updateCookLog(
                id,
                {
                    servings_made: servings,
                    duration_min: duration,
                    rating: rating as any,
                    notes: notes,
                    addImagePaths: newImages,
                    removeImageIds: imagesToRemove,
                },
                async (paths) => {
                    for (const p of paths) {
                        try {
                            await remove(p);
                        } catch (e) {}
                    }
                },
            );

            // Refresh data and exit edit mode
            log = await getCookLog(id);
            newImages = [];
            imagesToRemove = [];
            isEditing = false;
        } catch (e) {
            console.error(e);
            alert("Failed to update history.");
        } finally {
            saving = false;
        }
    }

    async function handleDelete() {
        if (!confirm("Delete this cooking session?")) return;
        try {
            await deleteCookLog(id, async (paths) => {
                for (const p of paths) {
                    try {
                        await remove(p);
                    } catch (e) {}
                }
            });
            goto("/recipes/history");
        } catch (e) {
            console.error(e);
        }
    }

    function formatDate(dateStr: string) {
        return new Date(dateStr).toLocaleDateString(undefined, {
            weekday: "long",
            year: "numeric",
            month: "long",
            day: "numeric",
        });
    }

    function cancelEdit() {
        if (log) {
            // Reset form state
            rating = log.rating || 5;
            notes = log.notes || "";
            servings = log.servings_made || 1;
            duration = log.duration_min || 0;
            newImages = [];
            imagesToRemove = [];
        }
        isEditing = false;
    }
</script>

<div class="min-h-screen bg-surface text-foreground pb-20">
    <!-- Header -->
    <div
        class="border-b border-line px-4 pt-[calc(1rem+env(safe-area-inset-top))] pb-4 mb-4 sticky top-0 z-10 flex justify-between items-center bg-surface shadow-sm"
    >
        <div class="flex items-center gap-2">
            {#if isEditing}
                <button
                    onclick={cancelEdit}
                    class="p-2 -ml-2 hover:bg-surface-sunken rounded-full transition text-foreground-muted"
                >
                    <X size={24} />
                </button>
            {:else}
                <button
                    onclick={() => goto("/recipes/history")}
                    class="p-2 -ml-2 hover:bg-surface-sunken rounded-full transition"
                >
                    <ChevronLeft size={24} />
                </button>
            {/if}
            <h1 class="text-xl font-bold">
                {isEditing ? "Edit Result" : "Cook Result"}
            </h1>
        </div>

        <div class="flex gap-2">
            {#if isEditing}
                <button
                    onclick={handleSave}
                    disabled={saving}
                    class="bg-accent text-background px-4 py-2 rounded-lg font-bold flex items-center gap-2 shadow-lg shadow-accent/20 hover:opacity-90 disabled:opacity-50 transition"
                >
                    {#if saving}
                        <Loader2 size={18} class="animate-spin" />
                    {:else}
                        <Save size={18} />
                    {/if}
                    Save
                </button>
            {:else}
                <button
                    onclick={handleDelete}
                    class="p-2 text-foreground-subtle hover:text-danger transition"
                >
                    <Trash2 size={20} />
                </button>
                <button
                    onclick={() => (isEditing = true)}
                    class="bg-accent text-background px-4 py-2 rounded-lg font-bold flex items-center gap-2 shadow-lg shadow-accent/20 hover:opacity-90 transition"
                >
                    <Edit2 size={18} />
                    Edit
                </button>
            {/if}
        </div>
    </div>

    {#if loading}
        <div
            class="flex flex-col items-center justify-center py-20 animate-pulse"
        >
            <div class="w-16 h-16 bg-surface-sunken rounded-2xl mb-4"></div>
            <div class="h-4 bg-surface-sunken w-32 rounded"></div>
        </div>
    {:else if log && recipe}
        <main
            class="px-4 max-w-2xl mx-auto space-y-8 animate-in fade-in duration-500"
        >
            <header class="text-center space-y-2">
                <div
                    class="inline-flex items-center gap-2 bg-success-soft text-success px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider mb-2"
                >
                    <CheckCircle2 size={14} />
                    {isEditing ? "Editing Session" : "Session Completed"}
                </div>
                <h2 class="text-2xl font-bold">{recipe.title}</h2>
                <div
                    class="flex items-center justify-center gap-2 text-sm text-foreground-muted font-medium"
                >
                    <Calendar size={14} />
                    {formatDate(log.created_at)}
                </div>
            </header>

            <!-- Gallery/Carousel -->
            <section class="space-y-4">
                <div class="flex items-center justify-between">
                    <h3
                        class="text-xs font-bold text-foreground-subtle uppercase tracking-widest"
                    >
                        Gallery
                    </h3>
                    {#if isEditing}
                        <button
                            onclick={pickImages}
                            class="text-accent text-xs font-bold flex items-center gap-1"
                        >
                            <Camera size={16} />
                            Add Photos
                        </button>
                    {/if}
                </div>

                <div class="flex gap-4 overflow-x-auto no-scrollbar pb-2">
                    {#each log.images as img}
                        <div
                            class="relative flex-shrink-0 w-48 aspect-square rounded-3xl border border-line overflow-hidden shadow-sm"
                        >
                            <img
                                src={convertFileSrc(img.file_path)}
                                alt="Dish"
                                class="w-full h-full object-cover {isEditing &&
                                imagesToRemove.includes(img.id)
                                    ? 'opacity-30 grayscale'
                                    : ''}"
                            />
                            {#if isEditing}
                                <button
                                    onclick={() => toggleRemoveImage(img.id)}
                                    class="absolute top-2 right-2 bg-black/50 text-white rounded-full p-1"
                                >
                                    {#if imagesToRemove.includes(img.id)}
                                        <RotateCcw size={14} />
                                    {:else}
                                        <X size={14} />
                                    {/if}
                                </button>
                            {/if}
                        </div>
                    {/each}
                    {#if isEditing}
                        {#each newImages as img, i}
                            <div
                                class="relative flex-shrink-0 w-48 aspect-square rounded-3xl border-2 border-accent border-dashed overflow-hidden shadow-sm"
                            >
                                <img
                                    src={convertFileSrc(img)}
                                    alt="New Dish"
                                    class="w-full h-full object-cover opacity-60"
                                />
                                <div
                                    class="absolute inset-0 flex items-center justify-center"
                                >
                                    <span
                                        class="bg-accent text-background text-[10px] font-bold px-2 py-1 rounded-full uppercase"
                                        >Pending</span
                                    >
                                </div>
                                <button
                                    onclick={() =>
                                        (newImages = newImages.filter(
                                            (_, idx) => idx !== i,
                                        ))}
                                    class="absolute top-2 right-2 bg-black/50 text-white rounded-full p-1"
                                >
                                    <X size={14} />
                                </button>
                            </div>
                        {/each}
                    {/if}

                    {#if isEditing && log.images.length === 0 && newImages.length === 0}
                        <button
                            onclick={pickImages}
                            class="w-full aspect-video bg-surface-sunken rounded-3xl border-2 border-dashed border-line flex flex-col items-center justify-center text-foreground-subtle gap-2 hover:bg-surface-raised transition"
                        >
                            <Camera size={32} strokeWidth={1.5} />
                            <span
                                class="text-xs font-bold uppercase tracking-wider"
                                >Add dish photos</span
                            >
                        </button>
                    {:else if !isEditing && log.images.length === 0}
                        <div
                            class="w-full aspect-video bg-surface-sunken rounded-3xl border border-line flex flex-col items-center justify-center text-foreground-subtle gap-2"
                        >
                            <Camera size={32} strokeWidth={1.5} />
                            <span
                                class="text-xs font-bold uppercase tracking-wider"
                                >No photos for this cook</span
                            >
                        </div>
                    {/if}
                </div>
            </section>

            <!-- Stats -->
            <section class="grid grid-cols-2 gap-4">
                <div
                    class="bg-surface p-4 rounded-3xl border border-line shadow-sm space-y-1"
                >
                    <label
                        class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1"
                    >
                        <ChefHat size={12} />
                        Servings
                    </label>
                    {#if isEditing}
                        <input
                            type="number"
                            bind:value={servings}
                            class="w-full bg-transparent border-none p-0 text-lg font-bold focus:ring-0"
                        />
                    {:else}
                        <div class="text-lg font-bold">{servings}</div>
                    {/if}
                </div>
                <div
                    class="bg-surface p-4 rounded-3xl border border-line shadow-sm space-y-1"
                >
                    <label
                        class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1"
                    >
                        <Clock size={12} />
                        Duration (m)
                    </label>
                    {#if isEditing}
                        <input
                            type="number"
                            bind:value={duration}
                            class="w-full bg-transparent border-none p-0 text-lg font-bold focus:ring-0"
                        />
                    {:else}
                        <div class="text-lg font-bold">{duration}m</div>
                    {/if}
                </div>
            </section>

            <!-- Rating -->
            <section
                class="bg-surface p-6 rounded-3xl border border-line shadow-sm text-center space-y-4"
            >
                <h3
                    class="text-xs font-bold text-foreground-subtle uppercase tracking-widest"
                >
                    {isEditing ? "Rate your cook" : "Your Rating"}
                </h3>
                <div class="flex justify-center gap-3">
                    {#each Array(5) as _, i}
                        <button
                            onclick={() => isEditing && (rating = i + 1)}
                            class="transition {isEditing
                                ? 'active:scale-90'
                                : 'cursor-default'}"
                        >
                            <Star
                                size={40}
                                class={i < rating
                                    ? "text-secondary fill-current"
                                    : "text-line"}
                            />
                        </button>
                    {/each}
                </div>
            </section>

            <!-- Notes -->
            <section class="space-y-3">
                <h3
                    class="text-xs font-bold text-foreground-subtle uppercase tracking-widest"
                >
                    Cooking Notes
                </h3>
                {#if isEditing}
                    <textarea
                        bind:value={notes}
                        placeholder="How was the texture? Any spices to add next time?"
                        rows="5"
                        class="w-full bg-surface border border-line rounded-3xl p-6 text-sm resize-none shadow-sm focus:ring-accent/20 focus:border-accent transition"
                    ></textarea>
                {:else if notes}
                    <div
                        class="bg-surface-sunken p-6 rounded-3xl border border-line text-sm leading-relaxed italic text-foreground-muted"
                    >
                        "{notes}"
                    </div>
                {:else}
                    <div
                        class="text-sm text-foreground-subtle italic text-center py-4 bg-surface-sunken rounded-3xl border border-line border-dashed"
                    >
                        No notes for this session.
                    </div>
                {/if}
            </section>
        </main>
    {/if}
</div>

<style>
    .no-scrollbar::-webkit-scrollbar {
        display: none;
    }
    .no-scrollbar {
        -ms-overflow-style: none;
        scrollbar-width: none;
    }
    input[type="number"]::-webkit-inner-spin-button,
    input[type="number"]::-webkit-outer-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }
    /* Simple scale animation for stars */
    button:active:not(.cursor-default) {
        transform: scale(0.9);
    }
</style>
