<script lang="ts">
    import { Drawer } from "vaul-svelte";
    import { onMount } from "svelte";
    import {
        getCookLog,
        updateCookLog,
        deleteCookLog,
    } from "$lib/services/cooklog";
    import type { RecipeHistoryWithImages } from "$lib/types";
    import {
        Star,
        Clock,
        Camera,
        X,
        ChefHat,
        Loader2,
        Trash2,
        Save,
        RotateCcw,
    } from "lucide-svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { remove } from "@tauri-apps/plugin-fs";
    import { saveImages } from "$lib/utils";

    let {
        open: isOpen = $bindable(false),
        historyId,
        onUpdated,
    }: { open: boolean; historyId: string; onUpdated: () => void } = $props();

    let log = $state<RecipeHistoryWithImages | null>(null);
    let loading = $state(true);
    let saving = $state(false);

    // Form state
    let rating = $state(5);
    let notes = $state("");
    let servings = $state(1);
    let duration = $state(0);
    let newImages = $state<string[]>([]);
    let imagesToRemove = $state<string[]>([]);

    $effect(() => {
        if (isOpen && historyId) {
            loadLog();
        }
    });

    async function loadLog() {
        loading = true;
        try {
            log = await getCookLog(historyId);
            if (log) {
                rating = log.rating || 5;
                notes = log.notes || "";
                servings = log.servings_made || 1;
                duration = log.duration_min || 0;
                newImages = [];
                imagesToRemove = [];
            }
        } catch (e) {
            console.error(e);
        } finally {
            loading = false;
        }
    }

    async function pickImages() {
        const selected = await saveImages("history_images");
        newImages = [...newImages, ...selected];
    }

    async function handleSave() {
        if (!log) return;
        saving = true;
        try {
            await updateCookLog(
                historyId,
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

            isOpen = false;
            onUpdated();
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
            await deleteCookLog(historyId, async (paths) => {
                for (const p of paths) {
                    try {
                        await remove(p);
                    } catch (e) {}
                }
            });
            isOpen = false;
            onUpdated();
        } catch (e) {
            console.error(e);
        }
    }

    function toggleRemoveImage(id: string) {
        if (imagesToRemove.includes(id)) {
            imagesToRemove = imagesToRemove.filter((i) => i !== id);
        } else {
            imagesToRemove = [...imagesToRemove, id];
        }
    }
</script>

<Drawer.Root bind:open={isOpen}>
    <Drawer.Portal>
        <Drawer.Overlay class="fixed inset-0 z-50 bg-black/40" />
        <Drawer.Content
            class="fixed bottom-0 left-0 right-0 z-50 mt-24 flex h-[90dvh] flex-col rounded-t-2xl bg-surface text-foreground shadow-lg border-t border-line outline-none"
        >
            <div
                class="mx-auto mt-4 h-1.5 w-12 shrink-0 rounded-full bg-line-strong"
            ></div>

            <div class="flex-1 overflow-y-auto p-6 space-y-8">
                <div class="flex items-center justify-between">
                    <Drawer.Title class="text-xl font-bold"
                        >Edit Cook Log</Drawer.Title
                    >
                    <button
                        onclick={handleDelete}
                        class="p-2 text-foreground-subtle hover:text-danger transition"
                    >
                        <Trash2 size={20} />
                    </button>
                </div>

                {#if loading}
                    <div class="flex justify-center py-20">
                        <Loader2 size={32} class="animate-spin text-accent" />
                    </div>
                {:else if log}
                    <!-- Rating -->
                    <section class="space-y-4">
                        <h3
                            class="text-xs font-bold text-foreground-subtle uppercase tracking-widest"
                        >
                            Rating
                        </h3>
                        <div class="flex gap-3">
                            {#each Array(5) as _, i}
                                <button
                                    onclick={() => (rating = i + 1)}
                                    class="transition active:scale-90"
                                >
                                    <Star
                                        size={32}
                                        class={i < rating
                                            ? "text-secondary fill-current"
                                            : "text-line"}
                                    />
                                </button>
                            {/each}
                        </div>
                    </section>

                    <!-- Gallery -->
                    <section class="space-y-4">
                        <div class="flex items-center justify-between">
                            <h3
                                class="text-xs font-bold text-foreground-subtle uppercase tracking-widest"
                            >
                                Photos
                            </h3>
                            <button
                                onclick={pickImages}
                                class="text-accent text-xs font-bold flex items-center gap-1"
                            >
                                <Camera size={16} />
                                Add
                            </button>
                        </div>

                        <div
                            class="flex gap-3 overflow-x-auto no-scrollbar pb-2"
                        >
                            {#each log.images as img}
                                <div
                                    class="relative shrink-0 w-24 aspect-square rounded-xl border border-line overflow-hidden shadow-sm"
                                >
                                    <img
                                        src={convertFileSrc(img.file_path)}
                                        alt="Dish"
                                        class="w-full h-full object-cover {imagesToRemove.includes(
                                            img.id,
                                        )
                                            ? 'opacity-30 grayscale'
                                            : ''}"
                                    />
                                    <button
                                        onclick={() =>
                                            toggleRemoveImage(img.id)}
                                        class="absolute top-1 right-1 bg-black/50 text-white rounded-full p-1"
                                    >
                                        {#if imagesToRemove.includes(img.id)}
                                            <RotateCcw size={12} />
                                        {:else}
                                            <X size={12} />
                                        {/if}
                                    </button>
                                </div>
                            {/each}
                            {#each newImages as img, i}
                                <div
                                    class="relative shrink-0 w-24 aspect-square rounded-xl border-2 border-accent border-dashed overflow-hidden shadow-sm"
                                >
                                    <img
                                        src={convertFileSrc(img)}
                                        alt="New Dish"
                                        class="w-full h-full object-cover opacity-60"
                                    />
                                    <button
                                        onclick={() =>
                                            (newImages = newImages.filter(
                                                (_, idx) => idx !== i,
                                            ))}
                                        class="absolute top-1 right-1 bg-black/50 text-white rounded-full p-1"
                                    >
                                        <X size={12} />
                                    </button>
                                </div>
                            {/each}
                        </div>
                    </section>

                    <!-- Stats -->
                    <section class="grid grid-cols-2 gap-4">
                        <div
                            class="bg-surface-sunken p-4 rounded-2xl border border-line space-y-1"
                        >
                            <label
                                class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1"
                            >
                                <ChefHat size={12} />
                                Servings
                            </label>
                            <input
                                type="number"
                                bind:value={servings}
                                class="w-full bg-transparent border-none p-0 text-lg font-bold focus:ring-0"
                            />
                        </div>
                        <div
                            class="bg-surface-sunken p-4 rounded-2xl border border-line space-y-1"
                        >
                            <label
                                class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1"
                            >
                                <Clock size={12} />
                                Min
                            </label>
                            <input
                                type="number"
                                bind:value={duration}
                                class="w-full bg-transparent border-none p-0 text-lg font-bold focus:ring-0"
                            />
                        </div>
                    </section>

                    <!-- Notes -->
                    <section class="space-y-3">
                        <h3
                            class="text-xs font-bold text-foreground-subtle uppercase tracking-widest"
                        >
                            Notes
                        </h3>
                        <textarea
                            bind:value={notes}
                            placeholder="Add your cooking notes..."
                            rows="4"
                            class="w-full bg-surface-sunken border border-line rounded-2xl p-4 text-sm resize-none focus:ring-accent/20 focus:border-accent transition"
                        ></textarea>
                    </section>
                {/if}
            </div>

            <div
                class="p-6 border-t border-line bg-surface flex gap-3 pb-[calc(1.5rem+env(safe-area-inset-bottom))]"
            >
                <Drawer.Close
                    class="flex-1 py-3 text-sm font-bold rounded-xl border border-line hover:bg-surface-raised transition"
                    disabled={saving}>Cancel</Drawer.Close
                >
                <button
                    onclick={handleSave}
                    disabled={saving || loading}
                    class="flex-2 bg-accent text-background py-3 rounded-xl font-bold flex items-center justify-center gap-2 shadow-lg shadow-accent/20 hover:opacity-90 disabled:opacity-50 transition"
                >
                    {#if saving}
                        <Loader2 size={18} class="animate-spin" />
                        Saving...
                    {:else}
                        <Save size={18} />
                        Save Changes
                    {/if}
                </button>
            </div>
        </Drawer.Content>
    </Drawer.Portal>
</Drawer.Root>

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
</style>
