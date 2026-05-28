<script lang="ts">
    import { theme, setTheme } from "$lib/stores/theme";
    import {
        ChevronLeft,
        Moon,
        Sun,
        Type,
        Palette,
        Info,
        ChefHat,
    } from "lucide-svelte";
    import { onMount } from "svelte";

    let fontSize = $state(16);

    onMount(() => {
        const storedFontSize = localStorage.getItem(
            "wait-im-cooking-font-size",
        );
        if (storedFontSize) {
            fontSize = parseInt(storedFontSize);
            applyFontSize(fontSize);
        }
    });

    function applyFontSize(size: number) {
        document.documentElement.style.fontSize = `${size}px`;
        localStorage.setItem("wait-im-cooking-font-size", size.toString());
    }

    $effect(() => {
        applyFontSize(fontSize);
    });
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
        <h1 class="text-xl font-bold ml-2">Settings</h1>
    </div>

    <div class="px-4 max-w-2xl mx-auto space-y-8">
        <!-- Appearance -->
        <section class="space-y-4">
            <div
                class="flex items-center gap-2 text-gray-500 font-bold text-sm uppercase tracking-wider"
            >
                <Palette size={16} />
                <span>Appearance</span>
            </div>

            <div
                class="bg-white rounded-2xl border border-line shadow-sm overflow-hidden divide-y divide-line"
            >
                <!-- Theme Selector -->
                <div class="p-4 flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div
                            class="w-10 h-10 rounded-xl bg-gray-50 flex items-center justify-center text-gray-500"
                        >
                            {#if $theme === "dark"}
                                <Moon size={20} />
                            {:else}
                                <Sun size={20} />
                            {/if}
                        </div>
                        <div>
                            <p class="font-bold">Dark Mode</p>
                            <p class="text-xs text-gray-500">
                                Switch between light and dark themes
                            </p>
                        </div>
                    </div>

                    <button
                        onclick={() =>
                            setTheme($theme === "dark" ? "light" : "dark")}
                        class="w-12 h-6 rounded-full relative transition-colors duration-200 focus:outline-none {$theme ===
                        'dark'
                            ? 'bg-accent'
                            : 'bg-gray-200'}"
                    >
                        <div
                            class="absolute top-1 left-1 w-4 h-4 rounded-full bg-white transition-transform duration-200 {$theme ===
                            'dark'
                                ? 'translate-x-6'
                                : ''}"
                        ></div>
                    </button>
                </div>

                <!-- Font Size -->
                <div class="p-4 space-y-4">
                    <div class="flex items-center gap-3">
                        <div
                            class="w-10 h-10 rounded-xl bg-gray-50 flex items-center justify-center text-gray-500"
                        >
                            <Type size={20} />
                        </div>
                        <div>
                            <p class="font-bold">Text Size</p>
                            <p class="text-xs text-gray-500">
                                Adjust the application font size
                            </p>
                        </div>
                    </div>

                    <div class="flex items-center gap-4 px-2">
                        <span class="text-xs text-gray-400">A</span>
                        <input
                            type="range"
                            min="12"
                            max="24"
                            step="1"
                            bind:value={fontSize}
                            class="flex-1 accent-accent h-1.5 bg-gray-100 rounded-lg appearance-none cursor-pointer"
                        />
                        <span class="text-lg text-gray-700 font-bold">A</span>
                    </div>
                    <p class="text-center text-xs text-gray-400 font-medium">
                        {fontSize}px
                    </p>
                </div>
            </div>
        </section>

        <!-- About / Metadata Section -->
        <section class="space-y-4">
            <div
                class="flex items-center gap-2 text-gray-500 font-bold text-sm uppercase tracking-wider"
            >
                <Info size={16} />
                <span>About</span>
            </div>

            <div
                class="bg-white rounded-2xl border border-line shadow-sm p-6 space-y-6"
            >
                <div class="flex flex-col items-center text-center space-y-3">
                    <div
                        class="w-16 h-16 bg-accent rounded-2xl flex items-center justify-center text-background shadow-lg shadow-accent/20"
                    >
                        <ChefHat size={32} />
                    </div>
                    <div>
                        <h3 class="font-bold text-lg">Wait, I'm cooking</h3>
                        <p class="text-sm text-gray-500">v0.1.0-alpha</p>
                    </div>
                </div>

                <div class="space-y-4 pt-4 border-t border-line">
                    <div class="flex justify-between text-sm">
                        <span class="text-gray-500">Project Lead</span>
                        <span class="font-bold">Felix</span>
                    </div>
                    <div class="flex justify-between text-sm">
                        <span class="text-gray-500">Course</span>
                        <span class="font-bold italic">SMM, P</span>
                    </div>
                    <div class="flex justify-between text-sm">
                        <span class="text-gray-500">Instructor</span>
                        <span class="font-bold">dr inż. Jakub Długosz</span>
                    </div>
                </div>

                <p
                    class="text-[10px] text-gray-400 text-center leading-relaxed"
                >
                    This project is a multimedia application designed for the
                    "SMM, P" course at Wroclaw University of Science and
                    Technology. All multimedia assets are either original or
                    used under appropriate free licenses.
                </p>
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
</style>
