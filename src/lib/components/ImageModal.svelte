<script lang="ts">
    import { X, ZoomIn, ZoomOut, Download } from "@lucide/svelte";
    import { fade, scale } from "svelte/transition";

    let {
        src,
        alt = "Image preview",
        onClose,
    }: {
        src: string | null;
        alt?: string;
        onClose: () => void;
    } = $props();

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Escape") onClose();
    }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if src}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
        class="fixed inset-0 z-100 flex items-center justify-center bg-black/95 backdrop-blur-sm p-4"
        transition:fade={{ duration: 200 }}
        onclick={onClose}
    >
        <button
            class="absolute top-[calc(1rem+env(safe-area-inset-top))] right-4 z-110 p-2 bg-white/10 hover:bg-white/20 rounded-full text-white transition"
            onclick={(e) => {
                e.stopPropagation();
                onClose();
            }}
        >
            <X size={24} />
        </button>

        <div
            class="relative max-w-full max-h-full flex items-center justify-center"
            transition:scale={{ duration: 300, start: 0.95 }}
            onclick={(e) => e.stopPropagation()}
        >
            <img
                {src}
                {alt}
                class="max-w-full max-h-[90dvh] object-contain rounded-lg shadow-2xl"
            />
        </div>
    </div>
{/if}
