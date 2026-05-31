<script lang="ts">
    import { Tag as TagIcon } from "@lucide/svelte";
    import type { Tag } from "$lib/types";

    let {
        selectedTagIds = $bindable([]),
        availableTags = [],
        onCreateTag,
    }: {
        selectedTagIds: string[];
        availableTags: Tag[];
        onCreateTag: (name: string) => Promise<void>;
    } = $props();

    let newTagName = $state("");

    function toggleTag(tagId: string) {
        if (selectedTagIds.includes(tagId)) {
            selectedTagIds = selectedTagIds.filter((id) => id !== tagId);
        } else {
            selectedTagIds = [...selectedTagIds, tagId];
        }
    }

    async function handleCreate() {
        const name = newTagName.trim();
        if (!name) return;
        await onCreateTag(name);
        newTagName = "";
    }
</script>

<section class="space-y-4">
    <div
        class="flex items-center gap-2 text-foreground-muted font-bold text-sm uppercase tracking-wider"
    >
        <TagIcon size={16} />
        <span>Tags</span>
    </div>
    <div
        class="bg-surface p-4 rounded-xl border border-line shadow-sm space-y-4"
    >
        <div class="flex flex-wrap gap-2">
            {#each availableTags as tag}
                <button
                    onclick={() => toggleTag(tag.id)}
                    class="px-3 py-1 rounded-full text-xs font-medium border transition {selectedTagIds.includes(
                        tag.id,
                    )
                        ? 'bg-accent text-background border-accent'
                        : 'bg-surface-sunken text-foreground-muted border-line hover:border-accent/50'}"
                >
                    {tag.name}
                </button>
            {/each}
            {#if availableTags.length === 0}
                <p class="text-xs text-foreground-subtle italic">
                    No tags created yet.
                </p>
            {/if}
        </div>
        <div class="flex gap-2">
            <input
                type="text"
                bind:value={newTagName}
                placeholder="New tag name..."
                class="flex-1 bg-surface-sunken border border-line rounded-lg px-3 py-1.5 text-sm outline-none focus:ring-1 focus:ring-accent/20"
                onkeydown={(e) => e.key === "Enter" && handleCreate()}
            />
            <button
                onclick={handleCreate}
                class="px-3 py-1.5 bg-surface-sunken text-foreground-muted rounded-lg text-sm font-bold hover:bg-surface-raised transition"
            >
                Add
            </button>
        </div>
    </div>
</section>
