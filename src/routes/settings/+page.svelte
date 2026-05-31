<script lang="ts">
    import { theme, setTheme } from "$lib/stores/theme";
    import {
        settings,
        updateFontScale,
        updateLeftHandedMode,
        updatePickedVideo,
        updatePickedMusic,
    } from "$lib/stores/settings";
    import {
        ChevronLeft,
        Moon,
        Sun,
        Type,
        Palette,
        Info,
        ChefHat,
        Hand,
        Tag as TagIcon,
        ChevronRight,
        Video,
        Music,
        Trash2,
        Play,
    } from "@lucide/svelte";
    import { onMount } from "svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { saveFiles } from "$lib/utils";

    let fontSizePercent = $state(100);
    let isAnimating = $state(false);

    onMount(() => {
        const unsubscribe = settings.subscribe((value) => {
            fontSizePercent = value.fontScale;
        });
        return unsubscribe;
    });

    $effect(() => {
        updateFontScale(fontSizePercent);
    });

    async function pickVideo() {
        const results = await saveFiles("multimedia", [
            { name: "Video", extensions: ["mp4", "webm", "mkv"] },
        ]);
        if (results.length > 0) {
            updatePickedVideo(results[0]);
        }
    }

    async function pickMusic() {
        const results = await saveFiles("multimedia", [
            { name: "Music", extensions: ["mp3", "wav", "ogg", "flac"] },
        ]);
        if (results.length > 0) {
            updatePickedMusic(results[0]);
        }
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
        <h1 class="text-xl font-bold ml-2">Settings</h1>
    </div>

    <div class="px-4 max-w-2xl mx-auto space-y-8">
        <!-- Appearance -->
        <section class="space-y-4">
            <div
                class="flex items-center gap-2 text-foreground-muted font-bold text-sm uppercase tracking-wider"
            >
                <Palette size={16} />
                <span>Appearance</span>
            </div>

            <div
                class="bg-surface rounded-2xl border border-line shadow-sm overflow-hidden divide-y divide-line"
            >
                <!-- Theme Selector -->
                <div class="p-4 flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div
                            class="w-10 h-10 rounded-xl bg-surface-sunken flex items-center justify-center text-foreground-muted"
                        >
                            {#if $theme === "dark"}
                                <Moon size={20} />
                            {:else}
                                <Sun size={20} />
                            {/if}
                        </div>
                        <div>
                            <p class="font-bold">Dark Mode</p>
                            <p class="text-xs text-foreground-muted">
                                Switch between light and dark themes
                            </p>
                        </div>
                    </div>

                    <button
                        aria-label="Toggle dark mode"
                        onclick={() =>
                            setTheme($theme === "dark" ? "light" : "dark")}
                        class="w-12 h-6 rounded-full relative transition-colors duration-200 focus:outline-none {$theme ===
                        'dark'
                            ? 'bg-accent'
                            : 'bg-line'}"
                    >
                        <div
                            class="absolute top-1 left-1 w-4 h-4 rounded-full bg-background transition-transform duration-200 {$theme ===
                            'dark'
                                ? 'translate-x-6'
                                : ''}"
                        ></div>
                    </button>
                </div>

                <div class="p-4 flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div
                            class="w-10 h-10 rounded-xl bg-surface-sunken flex items-center justify-center text-foreground-muted"
                        >
                            <Hand size={20} />
                        </div>
                        <div>
                            <p class="font-bold">Left-handed Mode</p>
                            <p class="text-xs text-foreground-muted">
                                Move action buttons to the left side
                            </p>
                        </div>
                    </div>

                    <button
                        aria-label="Toggle left-handed mode"
                        onclick={() =>
                            updateLeftHandedMode(!$settings.leftHandedMode)}
                        class="w-12 h-6 rounded-full relative transition-colors duration-200 focus:outline-none {$settings.leftHandedMode
                            ? 'bg-accent'
                            : 'bg-line'}"
                    >
                        <div
                            class="absolute top-1 left-1 w-4 h-4 rounded-full bg-background transition-transform duration-200 {$settings.leftHandedMode
                                ? 'translate-x-6'
                                : ''}"
                        ></div>
                    </button>
                </div>

                <!-- Font Size -->
                <div class="p-4 space-y-4">
                    <div class="flex items-center gap-3">
                        <div
                            class="w-10 h-10 rounded-xl bg-surface-sunken flex items-center justify-center text-foreground-muted"
                        >
                            <Type size={20} />
                        </div>
                        <div>
                            <p class="font-bold">Text Size</p>
                            <p class="text-xs text-foreground-muted">
                                Scale application text size
                            </p>
                        </div>
                    </div>

                    <div class="flex items-center gap-4 px-2">
                        <span class="text-xs text-foreground-subtle">80%</span>
                        <input
                            type="range"
                            min="80"
                            max="150"
                            step="5"
                            bind:value={fontSizePercent}
                            class="flex-1 accent-accent h-1.5 bg-surface-sunken rounded-lg appearance-none cursor-pointer"
                        />
                        <span class="text-lg text-foreground font-bold"
                            >150%</span
                        >
                    </div>
                    <p
                        class="text-center text-xs text-foreground-subtle font-medium"
                    >
                        {fontSizePercent}%
                    </p>
                </div>
            </div>
        </section>

        <!-- General / Data Management -->
        <section class="space-y-4">
            <div
                class="flex items-center gap-2 text-foreground-muted font-bold text-sm uppercase tracking-wider"
            >
                <TagIcon size={16} />
                <span>General</span>
            </div>

            <div
                class="bg-surface rounded-2xl border border-line shadow-sm overflow-hidden divide-y divide-line"
            >
                <a
                    href="/settings/tags"
                    class="p-4 flex items-center justify-between hover:bg-surface-raised transition group"
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="w-10 h-10 rounded-xl bg-surface-sunken flex items-center justify-center text-foreground-muted group-hover:text-accent transition-colors"
                        >
                            <TagIcon size={20} />
                        </div>
                        <div>
                            <p class="font-bold">Recipe Tags</p>
                            <p class="text-xs text-foreground-muted">
                                Manage categories and labels
                            </p>
                        </div>
                    </div>
                    <ChevronRight
                        size={18}
                        class="text-line-strong group-hover:text-accent transition"
                    />
                </a>
            </div>
        </section>

        <!-- About / Metadata Section -->
        <section class="space-y-4">
            <div
                class="flex items-center gap-2 text-foreground-muted font-bold text-sm uppercase tracking-wider"
            >
                <Info size={16} />
                <span>About</span>
            </div>

            <div
                class="bg-surface rounded-2xl border border-line shadow-sm p-6 space-y-6"
            >
                <div class="flex flex-col items-center text-center space-y-3">
                    <button
                        onclick={() => (isAnimating = !isAnimating)}
                        class="w-16 h-16 bg-accent rounded-2xl flex items-center justify-center text-background shadow-lg shadow-accent/20 active:scale-90 transition-transform"
                    >
                        <ChefHat
                            size={32}
                            class={isAnimating ? "super-animation" : ""}
                        />
                    </button>
                    <div>
                        <h3 class="font-bold text-lg">Wait, I'm Cooking!</h3>
                        <p class="text-sm text-foreground-muted">
                            v0.1.0-alpha
                        </p>
                    </div>
                </div>

                <div class="space-y-4 pt-4 border-t border-line">
                    <div class="flex justify-between text-sm">
                        <span class="text-foreground-muted">Author</span>
                        <span class="font-bold">Kacper Borys</span>
                    </div>
                </div>

                <!-- Multimedia Assignment Section -->
                <div class="space-y-4 pt-6 border-t border-line">
                    <p
                        class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest text-center"
                    >
                        Assignment Multimedia
                    </p>

                    <div class="space-y-5">
                        <!-- Video Picker -->
                        <div class="flex flex-col gap-2">
                            <div class="flex items-center justify-between">
                                <div class="flex items-center gap-2">
                                    <Video size={16} class="text-accent" />
                                    <span class="text-sm font-bold"
                                        >Selected Video</span
                                    >
                                </div>
                                <button
                                    onclick={pickVideo}
                                    class="text-xs font-bold text-accent hover:underline"
                                >
                                    {$settings.pickedVideo
                                        ? "Change"
                                        : "Pick Video"}
                                </button>
                            </div>

                            {#if $settings.pickedVideo}
                                <div
                                    class="bg-surface-sunken p-3 rounded-xl border border-line flex items-center justify-between group"
                                >
                                    <span
                                        class="text-xs text-foreground-muted truncate flex-1"
                                    >
                                        {$settings.pickedVideo.split("/").pop()}
                                    </span>
                                    <button
                                        onclick={() => updatePickedVideo(null)}
                                        class="p-1 text-foreground-subtle hover:text-danger transition opacity-0 group-hover:opacity-100"
                                    >
                                        <Trash2 size={14} />
                                    </button>
                                </div>
                                <div
                                    class="aspect-video w-full rounded-xl overflow-hidden bg-black border border-line shadow-inner"
                                >
                                    <!-- svelte-ignore a11y_media_has_caption -->
                                    <video
                                        src={convertFileSrc(
                                            $settings.pickedVideo,
                                        )}
                                        controls
                                        class="w-full h-full object-contain"
                                    ></video>
                                </div>
                            {:else}
                                <div
                                    class="bg-surface-sunken border border-dashed border-line rounded-xl py-8 flex flex-col items-center justify-center gap-2 text-foreground-subtle"
                                >
                                    <Video size={24} strokeWidth={1.5} />
                                    <span
                                        class="text-[10px] font-bold uppercase tracking-wider"
                                        >No video selected</span
                                    >
                                </div>
                            {/if}
                        </div>

                        <!-- Music Picker -->
                        <div class="flex flex-col gap-2">
                            <div class="flex items-center justify-between">
                                <div class="flex items-center gap-2">
                                    <Music size={16} class="text-accent" />
                                    <span class="text-sm font-bold"
                                        >Selected Music</span
                                    >
                                </div>
                                <button
                                    onclick={pickMusic}
                                    class="text-xs font-bold text-accent hover:underline"
                                >
                                    {$settings.pickedMusic
                                        ? "Change"
                                        : "Pick Music"}
                                </button>
                            </div>

                            {#if $settings.pickedMusic}
                                <div
                                    class="bg-surface-sunken p-3 rounded-xl border border-line flex items-center justify-between group"
                                >
                                    <span
                                        class="text-xs text-foreground-muted truncate flex-1"
                                    >
                                        {$settings.pickedMusic.split("/").pop()}
                                    </span>
                                    <button
                                        onclick={() => updatePickedMusic(null)}
                                        class="p-1 text-foreground-subtle hover:text-danger transition opacity-0 group-hover:opacity-100"
                                    >
                                        <Trash2 size={14} />
                                    </button>
                                </div>
                                <audio
                                    src={convertFileSrc($settings.pickedMusic)}
                                    controls
                                    class="w-full mt-1"
                                ></audio>
                            {:else}
                                <div
                                    class="bg-surface-sunken border border-dashed border-line rounded-xl py-4 flex flex-col items-center justify-center gap-2 text-foreground-subtle"
                                >
                                    <Music size={20} strokeWidth={1.5} />
                                    <span
                                        class="text-[10px] font-bold uppercase tracking-wider"
                                        >No music selected</span
                                    >
                                </div>
                            {/if}
                        </div>
                    </div>
                </div>
            </div>
        </section>
    </div>
</div>

<style>
    /* Range input styling for better cross-browser look if needed */
    input[type="range"]::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;
        width: 18px;
        height: 18px;
        background: var(--color-accent);
        border-radius: 50%;
        cursor: pointer;
        border: 2px solid white;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    :global(.super-animation) {
        display: block;
        animation: super-spin 5s ease-in-out infinite;
        animation-timing-function: cubic-bezier(0.33, 0.33, 1 0.33);
    }

    @keyframes super-spin {
        0% {
            transform: rotate(0deg) scale(1);
            filter: drop-shadow(0 0 0px var(--color-accent));
        }
        50% {
            transform: rotate(180deg) scale(1.2);
            filter: drop-shadow(0 0 8px var(--color-accent));
        }
        100% {
            transform: rotate(360deg) scale(1);
            filter: drop-shadow(0 0 0px var(--color-accent));
        }
    }
</style>
