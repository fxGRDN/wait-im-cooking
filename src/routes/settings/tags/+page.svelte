<script lang="ts">
    import { onMount } from "svelte";
    import {
        getTags,
        createTag,
        deleteTag,
        updateTag,
    } from "$lib/services/recipes";
    import type { Tag } from "$lib/types";
    import {
        ChevronLeft,
        Tag as TagIcon,
        Plus,
        Trash2,
        X,
        Loader2,
        Search,
        Edit2,
        Check,
    } from "@lucide/svelte";
    import { fade, slide } from "svelte/transition";

    let tags = $state<Tag[]>([]);
    let loading = $state(true);
    let newTagName = $state("");
    let saving = $state(false);
    let searchQuery = $state("");

    // Editing state
    let editingTagId = $state<string | null>(null);
    let editName = $state("");
    let updatingId = $state<string | null>(null);

    onMount(async () => {
        await loadTags();
    });

    async function loadTags() {
        try {
            tags = await getTags();
        } catch (e) {
            console.error(e);
        } finally {
            loading = false;
        }
    }

    async function handleAddTag() {
        const name = newTagName.trim();
        if (!name || saving) return;

        saving = true;
        try {
            const newTag = await createTag(name);
            tags = [...tags, newTag].sort((a, b) =>
                a.name.localeCompare(b.name),
            );
            newTagName = "";
        } catch (e) {
            console.error(e);
            alert("Failed to create tag.");
        } finally {
            saving = false;
        }
    }

    async function handleDeleteTag(tag: Tag) {
        if (
            !confirm(
                `Delete tag "${tag.name}"? This will remove it from all recipes.`,
            )
        )
            return;

        try {
            await deleteTag(tag.id);
            tags = tags.filter((t) => t.id !== tag.id);
        } catch (e) {
            console.error(e);
            alert("Failed to delete tag. It might still be in use.");
        }
    }

    function startEdit(tag: Tag) {
        editingTagId = tag.id;
        editName = tag.name;
    }

    function cancelEdit() {
        editingTagId = null;
        editName = "";
    }

    async function handleUpdateTag() {
        if (!editingTagId || !editName.trim() || updatingId) return;

        updatingId = editingTagId;
        try {
            await updateTag(editingTagId, editName.trim());
            tags = tags.map((t) =>
                t.id === editingTagId ? { ...t, name: editName.trim() } : t,
            );
            editingTagId = null;
            editName = "";
        } catch (e) {
            console.error(e);
            alert("Failed to rename tag.");
        } finally {
            updatingId = null;
        }
    }

    let filteredTags = $derived(
        tags.filter((t) =>
            t.name.toLowerCase().includes(searchQuery.toLowerCase()),
        ),
    );
</script>

<div class="min-h-screen bg-surface text-foreground pb-20">
    <!-- Header -->
    <div
        class="border-b border-line px-4 pt-[calc(1rem+env(safe-area-inset-top))] pb-4 mb-4 sticky top-0 z-10 flex items-center bg-surface shadow-sm"
    >
        <button
            onclick={() => history.back()}
            class="p-2 -ml-2 hover:bg-surface-sunken rounded-full transition"
        >
            <ChevronLeft size={24} />
        </button>
        <div class="flex items-center gap-2 ml-2">
            <TagIcon size={20} class="text-accent" />
            <h1 class="text-xl font-bold">Manage Tags</h1>
        </div>
    </div>

    <div class="px-4 max-w-2xl mx-auto space-y-6">
        <!-- Add Tag Section -->
        <section
            class="bg-surface p-4 rounded-2xl border border-line shadow-sm space-y-4"
        >
            <div class="flex flex-col gap-2">
                <label
                    for="new-tag"
                    class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest px-1"
                >
                    Create New Tag
                </label>
                <div class="flex gap-2">
                    <input
                        id="new-tag"
                        type="text"
                        bind:value={newTagName}
                        placeholder="e.g. Italian, Spicy, Dinner..."
                        class="flex-1 bg-surface-sunken border border-line rounded-xl px-4 py-2.5 text-sm focus:ring-2 focus:ring-accent/20 focus:border-accent outline-none transition"
                        onkeydown={(e) => e.key === "Enter" && handleAddTag()}
                    />
                    <button
                        onclick={handleAddTag}
                        disabled={saving || !newTagName.trim()}
                        class="bg-accent text-background px-4 py-2.5 rounded-xl font-bold flex items-center gap-2 shadow-lg shadow-accent/20 hover:opacity-90 disabled:opacity-50 transition"
                    >
                        {#if saving}
                            <Loader2 size={18} class="animate-spin" />
                        {:else}
                            <Plus size={18} />
                        {/if}
                        Add
                    </button>
                </div>
            </div>
        </section>

        <!-- Search & List -->
        <div class="space-y-4">
            <div class="relative">
                <Search
                    size={16}
                    class="absolute left-3 top-1/2 -translate-y-1/2 text-foreground-subtle"
                />
                <input
                    type="text"
                    bind:value={searchQuery}
                    placeholder="Search tags..."
                    class="w-full pl-10 pr-10 py-2 bg-surface-sunken border border-line rounded-xl text-sm outline-none transition"
                />
                {#if searchQuery}
                    <button
                        onclick={() => (searchQuery = "")}
                        class="absolute right-3 top-1/2 -translate-y-1/2 text-foreground-subtle hover:text-foreground transition"
                    >
                        <X size={14} />
                    </button>
                {/if}
            </div>

            {#if loading}
                <div class="space-y-3">
                    {#each Array(5) as _}
                        <div
                            class="h-14 bg-surface-sunken rounded-xl animate-pulse"
                        ></div>
                    {/each}
                </div>
            {:else if filteredTags.length === 0}
                <div class="text-center py-20 text-foreground-muted">
                    <TagIcon size={48} class="mx-auto mb-4 opacity-10" />
                    <p>No tags found.</p>
                </div>
            {:else}
                <div class="grid grid-cols-1 gap-2" transition:fade>
                    {#each filteredTags as tag (tag.id)}
                        <div
                            class="flex items-center justify-between p-4 bg-surface rounded-xl border border-line shadow-sm group hover:border-accent/30 transition min-h-16"
                            transition:slide|local
                        >
                            <div class="flex-1 flex items-center gap-3">
                                <div
                                    class="w-8 h-8 rounded-lg bg-accent/10 shrink-0 flex items-center justify-center text-accent"
                                >
                                    <TagIcon size={16} />
                                </div>

                                {#if editingTagId === tag.id}
                                    <div
                                        class="flex-1 flex gap-2"
                                        transition:fade={{ duration: 150 }}
                                    >
                                        <input
                                            type="text"
                                            bind:value={editName}
                                            class="flex-1 bg-surface-sunken border border-accent/30 rounded-lg px-3 py-1 text-sm outline-none focus:ring-1 focus:ring-accent/20"
                                            onkeydown={(e) => {
                                                if (e.key === "Enter")
                                                    handleUpdateTag();
                                                if (e.key === "Escape")
                                                    cancelEdit();
                                            }}
                                        />
                                        <button
                                            onclick={handleUpdateTag}
                                            disabled={updatingId === tag.id ||
                                                !editName.trim()}
                                            class="p-1.5 text-success hover:bg-success-soft rounded-lg transition"
                                        >
                                            {#if updatingId === tag.id}
                                                <Loader2
                                                    size={18}
                                                    class="animate-spin"
                                                />
                                            {:else}
                                                <Check size={18} />
                                            {/if}
                                        </button>
                                        <button
                                            onclick={cancelEdit}
                                            class="p-1.5 text-foreground-subtle hover:bg-surface-sunken rounded-lg transition"
                                        >
                                            <X size={18} />
                                        </button>
                                    </div>
                                {:else}
                                    <span class="font-bold truncate"
                                        >{tag.name}</span
                                    >
                                {/if}
                            </div>

                            {#if editingTagId !== tag.id}
                                <div
                                    class="flex gap-1 opacity-0 group-hover:opacity-100 focus-within:opacity-100 transition-opacity"
                                >
                                    <button
                                        onclick={() => startEdit(tag)}
                                        class="p-2 text-foreground-subtle hover:text-accent hover:bg-accent/5 rounded-lg transition"
                                        title="Rename Tag"
                                    >
                                        <Edit2 size={18} />
                                    </button>
                                    <button
                                        onclick={() => handleDeleteTag(tag)}
                                        class="p-2 text-foreground-subtle hover:text-danger hover:bg-danger/5 rounded-lg transition"
                                        title="Delete Tag"
                                    >
                                        <Trash2 size={18} />
                                    </button>
                                </div>
                            {/if}
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
    </div>
</div>
