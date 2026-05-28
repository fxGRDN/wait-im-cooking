<script lang="ts">
    import { onMount } from "svelte";
    import { getDb, isTauriRuntime } from "$lib/db/index";
    import { initTheme, theme } from "$lib/stores/theme";
    import BottomNav from "$lib/components/BottomNav.svelte";
    import TopLoadingBar from "$lib/components/TopLoadingBar.svelte";
    import "./layout.css";
    let { children } = $props();

    onMount(() => {
        if (isTauriRuntime()) {
            getDb();
        }
        initTheme();

        const unsubscribe = theme.subscribe((value) => {
            document.documentElement.dataset.theme = value;
            document.documentElement.style.colorScheme = value;
        });

        return unsubscribe;
    });
</script>

<TopLoadingBar />
<div class="flex min-h-dvh flex-col">
    <main class="flex-1">
        {@render children()}
    </main>
    <BottomNav />
</div>
