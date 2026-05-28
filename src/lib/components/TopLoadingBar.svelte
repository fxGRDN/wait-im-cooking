<script lang="ts">
    import { beforeNavigate, afterNavigate } from "$app/navigation";
    import ProgressBar from "svelte-progress-bar";

    let progress: any = $state();

    beforeNavigate(() => {
        progress?.start();
    });

    afterNavigate(() => {
        progress?.complete();
    });
</script>

<ProgressBar bind:this={progress} color="#c8753a" />

<style>
    :global(.svelte-progress-bar) {
        z-index: 100;
    }
    :global(.svelte-progress-bar-leader) {
        z-index: 101;
    }
</style>
