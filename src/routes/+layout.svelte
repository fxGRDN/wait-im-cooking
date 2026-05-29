<script lang="ts">
    import { onMount } from "svelte";
    import { initTheme, theme } from "$lib/stores/theme";
    import { initSettings } from "$lib/stores/settings";
    import BottomNav from "$lib/components/BottomNav.svelte";
    import TopLoadingBar from "$lib/components/TopLoadingBar.svelte";
    import "./layout.css";
    let { children } = $props();

    onMount(() => {
        console.log("App initialized, theme and settings loading...");
        initTheme();
        initSettings();

        const unsubscribe = theme.subscribe((value) => {
            document.documentElement.dataset.theme = value;
            document.documentElement.style.colorScheme = value;
        });

        return unsubscribe;
    });
</script>

<TopLoadingBar />
<div class="flex min-h-dvh flex-col bg-surface">
    <div class="flex-1 flex flex-col w-full max-w-2xl mx-auto relative">
        <main class="flex-1">
            {@render children()}
        </main>
    </div>
    <BottomNav />
</div>
