<script lang="ts">
    import type { Recipe } from "$lib/types";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { ChefHat, Clock, Heart, CheckCircle2 } from "lucide-svelte";
    import Badge from "./Badge.svelte";

    let {
        recipe,
        isCookable = false,
        class: className = "",
    } = $props<{
        recipe: Recipe;
        isCookable?: boolean;
        class?: string;
    }>();

    let imageError = $state(false);

    function formatTime(minutes: number | null) {
        if (!minutes) return null;
        if (minutes < 60) return `${minutes}m`;
        const h = Math.floor(minutes / 60);
        const m = minutes % 60;
        return m > 0 ? `${h}h ${m}m` : `${h}h`;
    }
</script>

<a
    href="/recipes/{recipe.id}"
    class="bg-surface rounded-2xl border border-line shadow-sm overflow-hidden hover:border-accent/50 transition-all flex group h-28 sm:h-32 {className}"
>
    {#if recipe.cover_image && !imageError}
        <div class="w-28 h-full sm:w-32 shrink-0 overflow-hidden">
            <img
                src={convertFileSrc(recipe.cover_image)}
                alt={recipe.title}
                onerror={() => (imageError = true)}
                class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-500"
            />
        </div>
    {:else}
        <div
            class="w-28 h-full sm:w-32 shrink-0 bg-surface-sunken flex items-center justify-center text-foreground-subtle"
        >
            <ChefHat size={32} strokeWidth={1.5} />
        </div>
    {/if}

    <div class="p-3 sm:p-4 flex-1 flex flex-col justify-between min-w-0">
        <div>
            <div class="flex justify-between items-start gap-2">
                <div class="flex flex-col gap-1 min-w-0">
                    <h2
                        class="text-sm sm:text-base font-bold group-hover:text-accent transition truncate"
                    >
                        {recipe.title}
                    </h2>
                    <div class="flex flex-wrap gap-1 mt-0.5">
                        {#if isCookable}
                            <Badge
                                label="Can Cook"
                                icon={CheckCircle2}
                                variant="success"
                                class="animate-in fade-in slide-in-from-left-2 duration-500"
                            />
                        {/if}
                        {#each recipe.tags as tag}
                            <Badge label={tag.name} variant="default" />
                        {/each}
                    </div>
                </div>
                {#if recipe.is_favourite}
                    <Heart
                        size={16}
                        class="text-danger fill-current shrink-0"
                    />
                {/if}
            </div>
            {#if recipe.description}
                <p
                    class="text-xs text-foreground-muted line-clamp-2 mt-0.5 leading-snug"
                >
                    {recipe.description}
                </p>
            {/if}
        </div>

        <div
            class="flex items-center gap-3 text-[10px] sm:text-xs font-bold text-foreground-subtle uppercase tracking-wider"
        >
            {#if recipe.prep_time}
                <span class="flex items-center gap-1">
                    <Clock size={12} />
                    {formatTime(recipe.prep_time)}
                </span>
            {/if}
            {#if recipe.cook_time}
                <span class="flex items-center gap-1">
                    <ChefHat size={12} />
                    {formatTime(recipe.cook_time)}
                </span>
            {/if}
        </div>
    </div>
</a>
