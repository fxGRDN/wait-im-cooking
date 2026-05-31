<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { page } from "$app/stores";
    import {
        getRecipeWithTree,
        deleteRecipe,
        toggleFavourite,
        updateRecipe,
        getTags,
        createTag,
        getRecipes,
    } from "$lib/services/recipes";
    import { getIngredients } from "$lib/services/ingredients";
    import { getCookLogs } from "$lib/services/cooklog";
    import { goto } from "$app/navigation";
    import type {
        RecipeWithTree,
        Ingredient,
        Recipe,
        Tag,
        RecipeIngredientInput,
        RecipeComponentInput,
        StepInput,
    } from "$lib/types";
    import {
        ChevronLeft,
        Clock,
        ChefHat,
        Users,
        Heart,
        Trash2,
        Edit2,
        Save,
        X,
        Camera,
        Plus,
        Info,
        Tag as TagIcon,
        Loader2,
        CheckCircle2,
    } from "@lucide/svelte";

    import { convertFileSrc } from "@tauri-apps/api/core";
    import { saveImages } from "$lib/utils";
    import ImageModal from "$lib/components/ImageModal.svelte";
    import RecipeIngredientsEditor from "$lib/components/RecipeIngredientsEditor.svelte";
    import RecipeStepsEditor from "$lib/components/RecipeStepsEditor.svelte";
    import RecipeTagsEditor from "$lib/components/RecipeTagsEditor.svelte";
    import RecipeComponentsEditor from "$lib/components/RecipeComponentsEditor.svelte";

    let recipe = $state<RecipeWithTree | null>(null);
    let loading = $state(true);
    let error = $state<string | null>(null);
    let isEditing = $state(false);
    let saving = $state(false);

    let historyNotes = $state<string[]>([]);

    // Form state
    let editTitle = $state("");
    let editDescription = $state("");
    let editServings = $state<number | null>(null);
    let editPrepTime = $state<number | null>(null);
    let editCookTime = $state<number | null>(null);
    let editCoverImage = $state<string | null>(null);
    let editIngredients = $state<RecipeIngredientInput[]>([]);
    let editComponents = $state<RecipeComponentInput[]>([]);
    let editSteps = $state<StepInput[]>([]);
    let editTagIds = $state<string[]>([]);
    let isFavourite = $state(false);

    // Selection state
    let availableIngredients = $state<Ingredient[]>([]);
    let availableRecipes = $state<Recipe[]>([]);
    let availableTags = $state<Tag[]>([]);

    let previewImage = $state<string | null>(null);
    const id = $page.params.id;

    onMount(async () => {
        await Promise.all([loadRecipe(), loadSelections()]);
        window.addEventListener("start-cooking", startCooking);
    });

    onDestroy(() => {
        window.removeEventListener("start-cooking", startCooking);
    });
    async function loadRecipe() {
        try {
            if (!id) {
                error = "Id not found";
                return;
            }

            const [recipeData, logs] = await Promise.all([
                getRecipeWithTree(id),
                getCookLogs(id),
            ]);

            recipe = recipeData;
            if (recipe) {
                // Extract last 3 notes
                historyNotes = logs
                    .map((l) => l.notes)
                    .filter((n) => n && n.trim().length > 0)
                    .slice(0, 3) as string[];

                // Initialize form state
                syncFormWithRecipe();
            } else {
                error = "Recipe not found";
            }
        } catch (e) {
            console.error(e);
            error = "Failed to load recipe";
        } finally {
            loading = false;
        }
    }

    function syncFormWithRecipe() {
        if (!recipe) return;
        editTitle = recipe.title;
        editDescription = recipe.description || "";
        editServings = recipe.servings;
        editPrepTime = recipe.prep_time;
        editCookTime = recipe.cook_time;
        editCoverImage = recipe.cover_image;
        editIngredients = recipe.ingredients.map((ri) => ({
            ingredient_id: ri.ingredient_id,
            quantity: ri.quantity,
            unit: ri.unit,
            is_optional: ri.is_optional,
        }));
        editComponents = recipe.components.map((rc) => ({
            child_id: rc.child_id,
            servings_needed: rc.servings_needed,
        }));
        editSteps = recipe.steps.map((s) => ({
            step_order: s.step_order,
            step_type: s.step_type,
            description: s.description,
            duration_min: s.duration_min,
        }));
        editTagIds = recipe.tags.map((t) => t.id);
        isFavourite = recipe.is_favourite;
    }

    async function loadSelections() {
        try {
            const [ings, recs, tags] = await Promise.all([
                getIngredients(),
                getRecipes(),
                getTags(),
            ]);
            availableIngredients = ings;
            // Don't allow adding current recipe as component
            availableRecipes = recs.filter((r) => r.id !== id);
            availableTags = tags;
        } catch (e) {
            console.error(e);
        }
    }

    const handleToggleFavourite = async () => {
        if (!recipe) return;
        try {
            await toggleFavourite(recipe.id);
            recipe.is_favourite = !recipe.is_favourite;
            isFavourite = recipe.is_favourite;
        } catch (e) {
            console.error(e);
        }
    };

    const handleDelete = async () => {
        if (!recipe || !confirm("Are you sure you want to delete this recipe?"))
            return;
        try {
            await deleteRecipe(recipe.id);
            goto("/recipes");
        } catch (e) {
            console.error(e);
            alert("Failed to delete recipe");
        }
    };

    async function handleSave() {
        if (!recipe) return;
        saving = true;
        try {
            const filteredIngredients = editIngredients.filter(
                (i) => i.ingredient_id,
            );
            const filteredComponents = editComponents.filter((c) => c.child_id);
            const filteredSteps = editSteps.filter((s) => s.description.trim());

            await updateRecipe(recipe.id, {
                title: editTitle,
                description: editDescription,
                servings: editServings,
                prep_time: editPrepTime,
                cook_time: editCookTime,
                is_favourite: isFavourite,
                cover_image: editCoverImage,
                ingredients: filteredIngredients,
                components: filteredComponents,
                steps: filteredSteps,
                tag_ids: editTagIds,
            });

            // Reload data
            await loadRecipe();
            isEditing = false;
        } catch (e) {
            console.error(e);
            alert("Failed to save changes.");
        } finally {
            saving = false;
        }
    }

    function cancelEdit() {
        syncFormWithRecipe();
        isEditing = false;
    }

    async function pickCover() {
        const savedPath = await saveImages("recipe_covers");
        if (savedPath && savedPath.length > 0) {
            editCoverImage = savedPath[0];
        }
    }

    const startCooking = () => {
        goto(`/recipes/${id}/cook`);
    };

    function formatTime(minutes: number | null) {
        if (!minutes) return null;
        if (minutes < 60) return `${minutes}m`;
        const h = Math.floor(minutes / 60);
        const m = minutes % 60;
        return m > 0 ? `${h}h ${m}m` : `${h}h`;
    }

    const onIngredientCreated = (newIng: Ingredient) => {
        availableIngredients = [...availableIngredients, newIng].sort((a, b) =>
            a.name.localeCompare(b.name),
        );
    };

    const onCreateTag = async (name: string) => {
        try {
            const tag = await createTag(name);
            availableTags = [...availableTags, tag];
            editTagIds = [...editTagIds, tag.id];
        } catch (e) {
            console.error(e);
        }
    };
</script>

<div class="min-h-screen bg-surface text-foreground pb-20">
    <!-- Header -->
    <div
        class="border-b border-line px-4 pt-[calc(1rem+env(safe-area-inset-top))] pb-4 mb-4 sticky top-0 z-20 flex justify-between items-center bg-surface shadow-sm"
    >
        <div class="flex items-center gap-2">
            {#if isEditing}
                <button
                    onclick={cancelEdit}
                    class="p-2 -ml-2 hover:bg-surface-sunken rounded-full transition text-foreground-muted"
                >
                    <X size={24} />
                </button>
            {:else}
                <button
                    onclick={() => history.back()}
                    class="p-2 -ml-2 hover:bg-surface-sunken rounded-full transition"
                >
                    <ChevronLeft size={24} />
                </button>
            {/if}
            <h1 class="text-xl font-bold truncate max-w-50">
                {isEditing ? "Edit Recipe" : recipe?.title || "Recipe"}
            </h1>
        </div>

        <div class="flex items-center gap-1">
            {#if isEditing}
                <button
                    onclick={handleSave}
                    disabled={saving || !editTitle.trim()}
                    class="bg-accent text-background px-4 py-2 rounded-lg font-bold flex items-center gap-2 shadow-lg shadow-accent/20 hover:opacity-90 disabled:opacity-50 transition"
                >
                    {#if saving}
                        <Loader2 size={18} class="animate-spin" />
                    {:else}
                        <Save size={18} />
                    {/if}
                    Save
                </button>
            {:else}
                <button
                    onclick={handleDelete}
                    class="p-2 text-foreground-subtle hover:text-danger transition"
                    title="Delete Recipe"
                >
                    <Trash2 size={20} />
                </button>
                <button
                    onclick={() => (isEditing = true)}
                    class="p-2 text-foreground-subtle hover:text-accent transition"
                    title="Edit Recipe"
                >
                    <Edit2 size={20} />
                </button>
                <button
                    onclick={handleToggleFavourite}
                    class="p-2 transition {recipe?.is_favourite
                        ? 'text-danger'
                        : 'text-foreground-subtle hover:text-danger'}"
                    title="Favourite"
                >
                    <Heart
                        size={22}
                        fill={recipe?.is_favourite ? "currentColor" : "none"}
                    />
                </button>
            {/if}
        </div>
    </div>

    {#if loading}
        <div
            class="flex flex-col items-center justify-center py-20 animate-pulse"
        >
            <div class="w-16 h-16 bg-surface-sunken rounded-2xl mb-4"></div>
            <div class="h-4 bg-surface-sunken w-32 rounded"></div>
        </div>
    {:else if error}
        <div class="px-4 py-12 text-center text-danger">
            <Info size={48} class="mx-auto mb-4 opacity-20" />
            <p class="font-bold">{error}</p>
            <button
                onclick={() => history.back()}
                class="mt-4 text-accent font-bold">Go Back</button
            >
        </div>
    {:else if recipe}
        <main
            class="px-4 max-w-2xl mx-auto space-y-8 animate-in fade-in duration-500"
        >
            <!-- Cover & Title -->
            <section class="space-y-4">
                {#if isEditing}
                    {#if editCoverImage}
                        <div
                            class="relative aspect-video rounded-2xl overflow-hidden border border-line shadow-sm group"
                        >
                            <img
                                src={convertFileSrc(editCoverImage)}
                                alt="Cover"
                                class="w-full h-full object-cover"
                            />
                            <div
                                class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center gap-2"
                            >
                                <button
                                    onclick={pickCover}
                                    class="p-3 bg-white/20 backdrop-blur-md rounded-full text-white hover:bg-white/40 transition"
                                >
                                    <Camera size={24} />
                                </button>
                                <button
                                    onclick={() => (editCoverImage = null)}
                                    class="p-3 bg-danger/20 backdrop-blur-md rounded-full text-danger-strong hover:bg-danger/40 transition"
                                >
                                    <X size={24} />
                                </button>
                            </div>
                        </div>
                    {:else}
                        <button
                            onclick={pickCover}
                            class="w-full aspect-video rounded-2xl border-2 border-dashed border-line hover:border-accent/50 hover:bg-accent/5 transition-all flex flex-col items-center justify-center gap-2 text-foreground-subtle"
                        >
                            <Camera size={32} strokeWidth={1.5} />
                            <span
                                class="text-sm font-bold uppercase tracking-widest"
                                >Change Cover Photo</span
                            >
                        </button>
                    {/if}
                {:else if recipe.cover_image}
                    <button
                        onclick={() =>
                            recipe?.cover_image &&
                            (previewImage = convertFileSrc(recipe.cover_image))}
                        class="w-full aspect-video rounded-3xl overflow-hidden border border-line shadow-md group"
                    >
                        <img
                            src={convertFileSrc(recipe.cover_image)}
                            alt={recipe.title}
                            class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-700"
                        />
                    </button>
                {/if}

                <div class="space-y-4">
                    <div
                        class="bg-surface p-6 rounded-3xl border border-line shadow-sm space-y-4"
                    >
                        {#if isEditing}
                            <div class="space-y-2">
                                <label
                                    class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest"
                                    for="edit-title">Recipe Title</label
                                >
                                <input
                                    id="edit-title"
                                    type="text"
                                    bind:value={editTitle}
                                    class="w-full bg-transparent border-none p-0 text-2xl font-bold focus:ring-0"
                                />
                            </div>
                            <div class="space-y-2">
                                <label
                                    class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest"
                                    for="edit-desc">Description</label
                                >
                                <textarea
                                    id="edit-desc"
                                    bind:value={editDescription}
                                    rows="2"
                                    class="w-full bg-transparent border-none p-0 text-sm focus:ring-0 resize-none"
                                ></textarea>
                            </div>
                        {:else}
                            <h2 class="text-3xl font-bold tracking-tight">
                                {recipe.title}
                            </h2>
                            {#if recipe.description}
                                <p
                                    class="text-foreground-muted leading-relaxed text-sm"
                                >
                                    {recipe.description}
                                </p>
                            {/if}
                        {/if}

                        <div class="flex flex-wrap gap-2">
                            {#if !isEditing}
                                {#each recipe.tags as tag}
                                    <span
                                        class="px-2.5 py-1 rounded-full text-[10px] font-bold bg-accent/5 text-accent border border-accent/10 uppercase tracking-wider"
                                    >
                                        {tag.name}
                                    </span>
                                {/each}
                            {/if}
                        </div>

                        <div
                            class="grid grid-cols-3 gap-4 pt-4 border-t border-line"
                        >
                            {#if isEditing}
                                <div class="space-y-1">
                                    <label
                                        class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1"
                                    >
                                        <ChefHat size={12} /> Servings
                                    </label>
                                    <input
                                        type="number"
                                        bind:value={editServings}
                                        class="w-full bg-surface-sunken border border-line rounded-xl px-3 py-2 text-sm font-bold focus:ring-1 focus:ring-accent/20 outline-none"
                                    />
                                </div>
                                <div class="space-y-1">
                                    <label
                                        class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1"
                                    >
                                        <Clock size={12} /> Prep (m)
                                    </label>
                                    <input
                                        type="number"
                                        bind:value={editPrepTime}
                                        class="w-full bg-surface-sunken border border-line rounded-xl px-3 py-2 text-sm font-bold focus:ring-1 focus:ring-accent/20 outline-none"
                                    />
                                </div>
                                <div class="space-y-1">
                                    <label
                                        class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1"
                                    >
                                        <Clock size={12} /> Cook (m)
                                    </label>
                                    <input
                                        type="number"
                                        bind:value={editCookTime}
                                        class="w-full bg-surface-sunken border border-line rounded-xl px-3 py-2 text-sm font-bold focus:ring-1 focus:ring-accent/20 outline-none"
                                    />
                                </div>
                            {:else}
                                <div
                                    class="flex flex-col items-center justify-center p-2 rounded-2xl bg-surface-sunken/50"
                                >
                                    <span
                                        class="text-[10px] font-bold text-foreground-subtle uppercase tracking-tighter mb-1"
                                        >Servings</span
                                    >
                                    <div
                                        class="flex items-center gap-1.5 text-accent"
                                    >
                                        <Users size={14} />
                                        <span class="font-bold text-sm"
                                            >{recipe.servings || "—"}</span
                                        >
                                    </div>
                                </div>
                                <div
                                    class="flex flex-col items-center justify-center p-2 rounded-2xl bg-surface-sunken/50"
                                >
                                    <span
                                        class="text-[10px] font-bold text-foreground-subtle uppercase tracking-tighter mb-1"
                                        >Prep Time</span
                                    >
                                    <div
                                        class="flex items-center gap-1.5 text-accent"
                                    >
                                        <Clock size={14} />
                                        <span class="font-bold text-sm"
                                            >{formatTime(recipe.prep_time) ||
                                                "—"}</span
                                        >
                                    </div>
                                </div>
                                <div
                                    class="flex flex-col items-center justify-center p-2 rounded-2xl bg-surface-sunken/50"
                                >
                                    <span
                                        class="text-[10px] font-bold text-foreground-subtle uppercase tracking-tighter mb-1"
                                        >Cook Time</span
                                    >
                                    <div
                                        class="flex items-center gap-1.5 text-accent"
                                    >
                                        <ChefHat size={14} />
                                        <span class="font-bold text-sm"
                                            >{formatTime(recipe.cook_time) ||
                                                "—"}</span
                                        >
                                    </div>
                                </div>
                            {/if}
                        </div>
                    </div>
                </div>
            </section>

            {#if isEditing}
                <RecipeTagsEditor
                    bind:selectedTagIds={editTagIds}
                    {availableTags}
                    {onCreateTag}
                />
            {/if}

            <!-- Ingredients -->
            <section class="space-y-4">
                {#if isEditing}
                    <RecipeIngredientsEditor
                        bind:ingredients={editIngredients}
                        {availableIngredients}
                        {onIngredientCreated}
                    />
                {:else}
                    <h3 class="text-lg font-bold flex items-center gap-2">
                        <ChefHat size={20} class="text-accent" />
                        Ingredients
                    </h3>
                    <div
                        class="bg-surface rounded-3xl border border-line shadow-sm divide-y divide-line overflow-hidden"
                    >
                        {#each recipe.ingredients as ri}
                            <div class="p-4 flex items-center justify-between">
                                <span class="font-medium"
                                    >{ri.ingredient.name}</span
                                >
                                <span class="text-foreground-muted font-bold"
                                    >{ri.quantity} {ri.unit}</span
                                >
                            </div>
                        {/each}
                    </div>
                {/if}
            </section>

            <!-- Components -->
            {#if isEditing || recipe.components.length > 0}
                <section class="space-y-4">
                    {#if isEditing}
                        <RecipeComponentsEditor
                            bind:components={editComponents}
                            {availableRecipes}
                        />
                    {:else}
                        <h3 class="text-lg font-bold flex items-center gap-2">
                            <Plus size={20} class="text-accent" />
                            Recipe Components
                        </h3>
                        <div class="grid grid-cols-1 gap-3">
                            {#each recipe.components as comp}
                                <a
                                    href="/recipes/{comp.child_id}"
                                    class="flex items-center justify-between bg-surface p-4 rounded-2xl border border-line shadow-sm hover:border-accent/50 transition-colors group"
                                >
                                    <div class="flex items-center gap-3">
                                        <div
                                            class="w-10 h-10 rounded-full bg-surface-sunken flex items-center justify-center text-accent"
                                        >
                                            <ChefHat size={20} />
                                        </div>
                                        <span
                                            class="font-bold group-hover:text-accent transition-colors"
                                            >{comp.child.title}</span
                                        >
                                    </div>
                                    <span
                                        class="text-xs font-bold text-foreground-subtle uppercase tracking-widest"
                                        >{comp.servings_needed} servings</span
                                    >
                                </a>
                            {/each}
                        </div>
                    {/if}
                </section>
            {/if}

            <!-- Steps -->
            <section class="space-y-4 pb-12">
                {#if isEditing}
                    <RecipeStepsEditor bind:steps={editSteps} />
                {:else}
                    <div class="flex items-center justify-between">
                        <h3 class="text-lg font-bold flex items-center gap-2">
                            <Clock size={20} class="text-accent" />
                            Steps
                        </h3>
                    </div>

                    <div class="space-y-0">
                        {#each recipe.steps as step, i}
                            <div class="flex gap-4">
                                <div class="flex flex-col items-center gap-2">
                                    <div
                                        class="w-8 h-8 rounded-full bg-accent text-background flex items-center justify-center font-bold shrink-0"
                                    >
                                        {i + 1}
                                    </div>
                                    {#if i < recipe.steps.length - 1}
                                        <div class="w-0.5 flex-1 bg-line"></div>
                                    {/if}
                                </div>
                                <div class="flex-1 pb-4">
                                    <div
                                        class="bg-surface p-4 rounded-2xl border border-line shadow-sm space-y-3"
                                    >
                                        <div
                                            class="flex justify-between items-start"
                                        >
                                            <span
                                                class="text-xs font-bold uppercase tracking-wider text-foreground-subtle"
                                            >
                                                {step.step_type}
                                            </span>
                                            {#if step.duration_min}
                                                <span
                                                    class="text-xs font-bold text-accent bg-accent/5 px-2 py-0.5 rounded-full"
                                                >
                                                    {formatTime(
                                                        step.duration_min,
                                                    )}
                                                </span>
                                            {/if}
                                        </div>
                                        <p
                                            class="text-foreground whitespace-pre-wrap text-sm"
                                        >
                                            {step.description}
                                        </p>
                                    </div>
                                </div>
                            </div>
                        {/each}
                    </div>
                {/if}
            </section>

            <!-- History Notes (Latest 3) -->
            {#if historyNotes.length > 0 && !isEditing}
                <section class="space-y-4 pb-12">
                    <h3 class="text-lg font-bold flex items-center gap-2">
                        <CheckCircle2 size={20} class="text-success" />
                        Recent Cooking Notes
                    </h3>
                    <div class="space-y-3">
                        {#each historyNotes as note}
                            <div
                                class="p-4 bg-surface rounded-2xl border border-line shadow-sm italic text-sm text-foreground-muted"
                            >
                                "{note}"
                            </div>
                        {/each}
                    </div>
                </section>
            {/if}
        </main>
    {/if}
</div>

<ImageModal bind:src={previewImage} onClose={() => (previewImage = null)} />
