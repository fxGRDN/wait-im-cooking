<script lang="ts">
    import { onMount } from "svelte";
    import { page } from "$app/state";
    import { resolve } from "$app/paths";
    import {
        House,
        Carrot,
        ChefHat,
        Settings,
        Plus,
        CheckCircle2,
    } from "@lucide/svelte";
    import { settings } from "$lib/stores/settings";

    const path = $derived(page.url.pathname);

    const homePath = resolve("/");
    const homeHref = homePath === "/" ? homePath : homePath.replace(/\/$/, "");
    const ingredientsPath = resolve("/ingredients");
    const recipesPath = resolve("/recipes");
    const settingsPath = resolve("/settings");

    const onHome = $derived(path === homeHref);
    const onIngredients = $derived(
        path === ingredientsPath || path.startsWith(ingredientsPath + "/"),
    );
    const onRecipes = $derived(
        path === recipesPath || path.startsWith(recipesPath + "/"),
    );
    const onSettings = $derived(
        path === settingsPath || path.startsWith(settingsPath + "/"),
    );

    // FAB State - relative to nav
    const showAddIngredient = $derived(path === ingredientsPath);
    const showAddRecipe = $derived(path === recipesPath);
    const showStartCooking = $derived(
        path.startsWith(recipesPath + "/") &&
            !path.startsWith(recipesPath + "/add") &&
            !path.startsWith(recipesPath + "/history") &&
            !path.endsWith("/cook") &&
            path.split("/").length === 3,
    );

    let isCookCompleted = $state(false);
    const showCompleteCooking = $derived(
        path.endsWith("/cook") && isCookCompleted,
    );

    onMount(() => {
        const handleStatus = (e: any) => {
            isCookCompleted = e.detail?.completed || false;
        };
        window.addEventListener("cook-status", handleStatus);
        return () => window.removeEventListener("cook-status", handleStatus);
    });

    const tabBase =
        "flex flex-col items-center gap-1 py-2 text-xs transition-all duration-300 ease-out outline-none focus-visible:ring-2 focus-visible:ring-accent-focus-ring rounded-lg mx-2 my-1";
    const tabActive = "text-accent bg-accent/10";
    const tabIdle =
        "text-foreground-muted hover:text-foreground hover:bg-surface-raised active:scale-95";
</script>

<nav
    class="sticky bottom-0 border-t border-line bg-surface/95 backdrop-blur-md pb-[env(safe-area-inset-bottom)] shadow-[0_-4px_20px_-10px_rgba(0,0,0,0.1)] z-50 touch-none overscroll-none select-none"
    aria-label="Primary"
>
    <!-- Relative FAB Container -->
    <div
        class="absolute bottom-[calc(100%+1rem)] left-0 right-0 pointer-events-none"
    >
        <div class="max-w-2xl mx-auto relative h-14">
            {#if showAddIngredient}
                <button
                    type="button"
                    onclick={() =>
                        window.dispatchEvent(
                            new CustomEvent("open-add-ingredient"),
                        )}
                    class="absolute w-14 h-14 bg-accent text-background rounded-full flex items-center justify-center shadow-lg hover:opacity-90 transition pointer-events-auto {$settings.leftHandedMode
                        ? 'left-6'
                        : 'right-6'}"
                    aria-label="Add to Inventory"
                >
                    <Plus size={28} strokeWidth={2.5} />
                </button>
            {:else if showAddRecipe}
                <a
                    href="/recipes/add"
                    class="absolute w-14 h-14 bg-accent text-background rounded-full flex items-center justify-center shadow-lg hover:opacity-90 transition pointer-events-auto {$settings.leftHandedMode
                        ? 'left-6'
                        : 'right-6'}"
                    aria-label="Add Recipe"
                >
                    <Plus size={28} strokeWidth={2.5} />
                </a>
            {:else if showStartCooking}
                <button
                    onclick={() =>
                        window.dispatchEvent(new CustomEvent("start-cooking"))}
                    class="absolute bg-accent text-background px-6 py-4 rounded-2xl font-bold shadow-xl shadow-accent/20 flex items-center gap-2 hover:scale-105 transition active:scale-95 pointer-events-auto {$settings.leftHandedMode
                        ? 'left-6'
                        : 'right-6'}"
                >
                    <ChefHat size={20} fill="currentColor" />
                    Start Cooking
                </button>
            {:else if showCompleteCooking}
                <button
                    onclick={() =>
                        window.dispatchEvent(
                            new CustomEvent("complete-cooking"),
                        )}
                    class="absolute bg-success text-white px-6 py-4 rounded-2xl font-bold shadow-xl shadow-success/20 flex items-center gap-2 hover:scale-105 transition active:scale-95 pointer-events-auto {$settings.leftHandedMode
                        ? 'left-6'
                        : 'right-6'}"
                >
                    <CheckCircle2 size={20} />
                    Complete
                </button>
            {/if}
        </div>
    </div>

    <ul class="mx-auto grid max-w-md grid-cols-4 gap-1 px-2 py-1">
        <li class="contents">
            <a
                href={homeHref}
                class="{tabBase} {onHome ? tabActive : tabIdle}"
                aria-current={onHome ? "page" : undefined}
            >
                <div
                    class="relative flex items-center justify-center h-7 w-7 mb-0.5 transition-transform duration-300 {onHome
                        ? 'scale-110'
                        : ''}"
                >
                    <House
                        class="h-5 w-5 transition-all duration-300 {onHome
                            ? 'stroke-[2.5px]'
                            : 'stroke-2'}"
                    />
                </div>
                <span class="font-semibold">Home</span>
            </a>
        </li>
        <li class="contents">
            <a
                href={ingredientsPath}
                class="{tabBase} {onIngredients ? tabActive : tabIdle}"
                aria-current={onIngredients ? "page" : undefined}
            >
                <div
                    class="relative flex items-center justify-center h-7 w-7 mb-0.5 transition-transform duration-300 {onIngredients
                        ? 'scale-110'
                        : ''}"
                >
                    <Carrot
                        class="h-5 w-5 transition-all duration-300 {onIngredients
                            ? 'stroke-[2.5px]'
                            : 'stroke-2'}"
                    />
                </div>
                <span class="font-semibold">Inventory</span>
            </a>
        </li>
        <li class="contents">
            <a
                href={recipesPath}
                class="{tabBase} {onRecipes ? tabActive : tabIdle}"
                aria-current={onRecipes ? "page" : undefined}
            >
                <div
                    class="relative flex items-center justify-center h-7 w-7 mb-0.5 transition-transform duration-300 {onRecipes
                        ? 'scale-110'
                        : ''}"
                >
                    <ChefHat
                        class="h-5 w-5 transition-all duration-300 {onRecipes
                            ? 'stroke-[2.5px]'
                            : 'stroke-2'}"
                    />
                </div>
                <span class="font-semibold">Recipes</span>
            </a>
        </li>
        <li class="contents">
            <a
                href={settingsPath}
                class="{tabBase} {onSettings ? tabActive : tabIdle}"
                aria-current={onSettings ? "page" : undefined}
            >
                <div
                    class="relative flex items-center justify-center h-7 w-7 mb-0.5 transition-transform duration-300 {onSettings
                        ? 'scale-110'
                        : ''}"
                >
                    <Settings
                        class="h-5 w-5 transition-all duration-300 {onSettings
                            ? 'stroke-[2.5px]'
                            : 'stroke-2'}"
                    />
                </div>
                <span class="font-semibold">Settings</span>
            </a>
        </li>
    </ul>
</nav>
