<script lang="ts">
    import { page } from "$app/state";
    import { resolve } from "$app/paths";
    import { House, Carrot, ChefHat } from "@lucide/svelte";

    const path = $derived(page.url.pathname);

    const homePath = resolve("/");
    const homeHref = homePath === "/" ? homePath : homePath.replace(/\/$/, "");
    const ingredientsPath = resolve("/ingredients");
    const recipesPath = resolve("/recipes");

    const onHome = $derived(path === homeHref);
    const onIngredients = $derived(path === ingredientsPath || path.startsWith(ingredientsPath + "/"));
    const onRecipes = $derived(path === recipesPath || path.startsWith(recipesPath + "/"));

    const tabBase = "flex flex-col items-center gap-1 py-2 text-xs transition-all duration-300 ease-out outline-none focus-visible:ring-2 focus-visible:ring-accent-focus-ring rounded-lg mx-2 my-1";
    const tabActive = "text-accent bg-accent/10";
    const tabIdle = "text-foreground-muted hover:text-foreground hover:bg-surface-raised active:scale-95";
</script>

<nav
    class="fixed inset-x-0 bottom-0 z-40 border-t border-line bg-surface/95 backdrop-blur-md pb-[env(safe-area-inset-bottom)] shadow-[0_-4px_20px_-10px_rgba(0,0,0,0.1)]"
    aria-label="Primary"
>
    <ul class="mx-auto grid max-w-md grid-cols-3 gap-1 px-2 py-1">
        <li class="contents">
            <a
                href={ingredientsPath}
                class="{tabBase} {onIngredients ? tabActive : tabIdle}"
                aria-current={onIngredients ? "page" : undefined}
            >
                <div class="relative flex items-center justify-center h-7 w-7 mb-0.5 transition-transform duration-300 {onIngredients ? 'scale-110' : ''}">
                    <Carrot class="h-5 w-5 transition-all duration-300 {onIngredients ? 'stroke-[2.5px]' : 'stroke-2'}" />
                </div>
                <span class="font-semibold">Inventory</span>
            </a>
        </li>
        <li class="contents">
            <a
                href={homeHref}
                class="{tabBase} {onHome ? tabActive : tabIdle}"
                aria-current={onHome ? "page" : undefined}
            >
                <div class="relative flex items-center justify-center h-7 w-7 mb-0.5 transition-transform duration-300 {onHome ? 'scale-110' : ''}">
                    <House class="h-5 w-5 transition-all duration-300 {onHome ? 'stroke-[2.5px]' : 'stroke-2'}" />
                </div>
                <span class="font-semibold">Home</span>
            </a>
        </li>
        <li class="contents">
            <a
                href={recipesPath}
                class="{tabBase} {onRecipes ? tabActive : tabIdle}"
                aria-current={onRecipes ? "page" : undefined}
            >
                <div class="relative flex items-center justify-center h-7 w-7 mb-0.5 transition-transform duration-300 {onRecipes ? 'scale-110' : ''}">
                    <ChefHat class="h-5 w-5 transition-all duration-300 {onRecipes ? 'stroke-[2.5px]' : 'stroke-2'}" />
                </div>
                <span class="font-semibold">Recipes</span>
            </a>
        </li>
    </ul>
</nav>
