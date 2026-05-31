<script lang="ts">
    import { onMount } from "svelte";
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
    } from "lucide-svelte";

    import { convertFileSrc } from "@tauri-apps/api/core";
    import { saveImages, COMMON_UNITS } from "$lib/utils";
    import QuickAddIngredient from "../add/QuickAddIngredient.svelte";
    import ImageModal from "$lib/components/ImageModal.svelte";

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

    // Selection state
    let availableIngredients = $state<Ingredient[]>([]);
    let availableRecipes = $state<Recipe[]>([]);
    let availableTags = $state<Tag[]>([]);
    let quickAddOpen = $state(false);
    let activeIngredientIndex = $state<number | null>(null);
    let newTagName = $state("");

    let previewImage = $state<string | null>(null);

    const id = $page.params.id;

    onMount(async () => {
        await Promise.all([loadRecipe(), loadSelections()]);
        window.addEventListener("start-cooking", startCooking);
        return () => window.removeEventListener("start-cooking", startCooking);
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

    const handleToggleFav = async () => {
        if (!recipe) return;
        try {
            await toggleFavourite(recipe.id);
            recipe.is_favourite = !recipe.is_favourite;
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
            await updateRecipe(recipe.id, {
                title: editTitle,
                description: editDescription,
                servings: editServings,
                prep_time: editPrepTime,
                cook_time: editCookTime,
                cover_image: editCoverImage,
                ingredients: editIngredients,
                components: editComponents,
                steps: editSteps,
                tag_ids: editTagIds,
            });

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
        if (recipe) {
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
        }
        isEditing = false;
    }

    async function pickCover() {
        const savedPath = await saveImages("recipe_covers");
        if (savedPath && savedPath.length > 0) {
            editCoverImage = savedPath[0];
        }
    }

    // Ingredient actions
    const addIngredient = () => {
        editIngredients.push({
            ingredient_id: "",
            quantity: 1,
            unit: COMMON_UNITS[0],
            is_optional: false,
        });
    };

    const removeIngredient = (index: number) => {
        editIngredients = editIngredients.filter((_, i) => i !== index);
    };

    const handleIngredientChange = (index: number, e: Event) => {
        const value = (e.target as HTMLSelectElement).value;
        if (value === "new") {
            activeIngredientIndex = index;
            quickAddOpen = true;
            editIngredients[index].ingredient_id = "";
        } else {
            const ing = availableIngredients.find((a) => a.id === value);
            if (ing?.default_unit && !editIngredients[index].unit) {
                editIngredients[index].unit = ing.default_unit;
            }
        }
    };

    const onIngredientCreated = (newIng: Ingredient) => {
        availableIngredients = [...availableIngredients, newIng].sort((a, b) =>
            a.name.localeCompare(b.name),
        );
        if (activeIngredientIndex !== null) {
            editIngredients[activeIngredientIndex].ingredient_id = newIng.id;
            editIngredients[activeIngredientIndex].unit =
                newIng.default_unit || COMMON_UNITS[0];
        }
        activeIngredientIndex = null;
    };

    // Component actions
    const addComponent = () => {
        editComponents.push({
            child_id: "",
            servings_needed: 1,
        });
    };

    const removeComponent = (index: number) => {
        editComponents = editComponents.filter((_, i) => i !== index);
    };

    // Step actions
    const addStep = (index?: number) => {
        const newStep = {
            step_order: 0,
            step_type: "prep" as const,
            description: "",
            duration_min: 0,
        };

        if (typeof index === "number") {
            editSteps.splice(index + 1, 0, newStep);
        } else {
            editSteps.push(newStep);
        }

        // Fix step orders
        editSteps = editSteps.map((s, i) => ({ ...s, step_order: i + 1 }));
    };

    const removeStep = (index: number) => {
        editSteps = editSteps.filter((_, i) => i !== index);
        // Fix step orders
        editSteps = editSteps.map((s, i) => ({ ...s, step_order: i + 1 }));
    };

    const moveStep = (index: number, direction: "up" | "down") => {
        if (direction === "up" && index > 0) {
            const temp = editSteps[index];
            editSteps[index] = editSteps[index - 1];
            editSteps[index - 1] = temp;
        } else if (direction === "down" && index < editSteps.length - 1) {
            const temp = editSteps[index];
            editSteps[index] = editSteps[index + 1];
            editSteps[index + 1] = temp;
        }
        // Fix step orders
        editSteps = editSteps.map((s, i) => ({ ...s, step_order: i + 1 }));
    };

    // Tag actions
    const toggleTag = (id: string) => {
        if (editTagIds.includes(id)) {
            editTagIds = editTagIds.filter((t) => t !== id);
        } else {
            editTagIds = [...editTagIds, id];
        }
    };

    const handleCreateTag = async () => {
        const name = newTagName.trim();
        if (!name) return;
        try {
            const newTag = await createTag(name);
            availableTags = [...availableTags, newTag];
            editTagIds = [...editTagIds, newTag.id];
            newTagName = "";
        } catch (e) {
            console.error(e);
        }
    };

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
</script>

<div class="min-h-screen bg-surface text-foreground pb-20">
    <!-- Header -->
    <div
        class="border-b border-line px-4 pt-[calc(1rem+env(safe-area-inset-top))] pb-4 mb-4 sticky top-0 z-10 flex justify-between items-center bg-surface shadow-sm"
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
                    disabled={saving}
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
                    onclick={handleToggleFav}
                    class="p-2 hover:bg-surface-sunken rounded-full transition {recipe?.is_favourite
                        ? 'text-danger'
                        : 'text-foreground-subtle'}"
                >
                    <Heart
                        size={20}
                        fill={recipe?.is_favourite ? "currentColor" : "none"}
                    />
                </button>
                <button
                    onclick={() => (isEditing = true)}
                    class="p-2 hover:bg-surface-sunken rounded-full transition text-foreground-subtle hover:text-accent"
                >
                    <Edit2 size={20} />
                </button>
                <button
                    onclick={handleDelete}
                    class="p-2 hover:bg-surface-sunken rounded-full transition text-foreground-subtle hover:text-danger"
                >
                    <Trash2 size={20} />
                </button>
            {/if}
        </div>
    </div>

    {#if loading}
        <div class="px-4 max-w-2xl mx-auto space-y-4 animate-pulse">
            <div class="h-48 bg-surface-sunken rounded-2xl w-full"></div>
            <div class="h-8 bg-surface-sunken rounded w-3/4"></div>
            <div class="h-4 bg-surface-sunken rounded w-1/2"></div>
        </div>
    {:else if error}
        <div class="px-4 text-center py-12">
            <p class="text-danger">{error}</p>
            <a href="/recipes" class="text-accent font-bold mt-4 inline-block"
                >Back to recipes</a
            >
        </div>
    {:else if recipe}
        <div
            class="px-4 max-w-2xl mx-auto space-y-8 animate-in fade-in duration-500"
        >
            <!-- Header/Meta -->
            <section class="space-y-6">
                {#if isEditing}
                    <div class="space-y-4">
                        <div class="relative group">
                            {#if editCoverImage}
                                <img
                                    src={convertFileSrc(editCoverImage)}
                                    alt="Cover"
                                    class="w-full h-48 sm:h-64 object-cover rounded-2xl shadow-sm border border-line"
                                />
                                <div
                                    class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 transition rounded-2xl flex items-center justify-center pointer-events-none"
                                >
                                    <Camera size={32} class="text-white" />
                                </div>
                                <button
                                    onclick={pickCover}
                                    class="absolute bottom-4 right-4 bg-surface p-2 rounded-full shadow-lg border border-line hover:bg-surface-raised transition"
                                >
                                    <Camera size={20} />
                                </button>
                                <button
                                    onclick={() => (editCoverImage = null)}
                                    class="absolute top-4 right-4 bg-danger text-white p-2 rounded-full shadow-lg hover:bg-danger/90 transition"
                                >
                                    <Trash2 size={20} />
                                </button>
                            {:else}
                                <button
                                    onclick={pickCover}
                                    class="w-full h-48 sm:h-64 bg-surface-sunken rounded-2xl border-2 border-dashed border-line flex flex-col items-center justify-center text-foreground-subtle gap-2 hover:bg-surface-raised transition"
                                >
                                    <Camera size={32} strokeWidth={1.5} />
                                    <span
                                        class="text-sm font-bold uppercase tracking-widest"
                                        >Add Cover Image</span
                                    >
                                </button>
                            {/if}
                        </div>

                        <div class="space-y-2">
                            <label
                                class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest px-1"
                                >Recipe Title</label
                            >
                            <input
                                type="text"
                                bind:value={editTitle}
                                placeholder="Recipe Title"
                                class="w-full bg-surface border border-line rounded-xl px-4 py-3 text-lg font-bold shadow-sm focus:ring-accent/20 focus:border-accent transition"
                            />
                        </div>

                        <div class="space-y-2">
                            <label
                                class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest px-1"
                                >Description</label
                            >
                            <textarea
                                bind:value={editDescription}
                                placeholder="Short description..."
                                rows="3"
                                class="w-full bg-surface border border-line rounded-xl px-4 py-3 text-sm shadow-sm focus:ring-accent/20 focus:border-accent transition resize-none"
                            ></textarea>
                        </div>
                    </div>
                {:else}
                    {#if recipe.cover_image}
                        <!-- svelte-ignore a11y_click_events_have_key_events -->
                        <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                        <img
                            src={convertFileSrc(recipe.cover_image)}
                            alt={recipe.title}
                            class="w-full h-48 sm:h-64 object-cover rounded-2xl shadow-sm border border-line cursor-zoom-in active:scale-[0.98] transition"
                            onclick={() =>
                                (previewImage = convertFileSrc(
                                    recipe!.cover_image!,
                                ))}
                        />
                    {/if}

                    <div class="space-y-2">
                        <h2 class="text-2xl font-bold">{recipe.title}</h2>
                        {#if recipe.description}
                            <p class="text-foreground-muted leading-relaxed">
                                {recipe.description}
                            </p>
                        {/if}
                    </div>
                {/if}

                {#if !isEditing && historyNotes.length > 0}
                    <div class="space-y-3 pt-2">
                        <h3
                            class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1 px-1"
                        >
                            <Info size={12} class="text-accent" />
                            Recent Notes from History
                        </h3>
                        <div class="space-y-2">
                            {#each historyNotes as note}
                                <div
                                    class="bg-surface-sunken p-3 rounded-xl border border-line text-sm text-foreground-muted italic leading-relaxed"
                                >
                                    "{note}"
                                </div>
                            {/each}
                        </div>
                    </div>
                {/if}

                <div
                    class="flex flex-wrap gap-4 py-4 border-y border-line text-sm font-medium text-foreground-muted"
                >
                    {#if isEditing}
                        <div
                            class="grid grid-cols-1 sm:grid-cols-3 gap-4 w-full"
                        >
                            <div
                                class="bg-surface p-3 rounded-xl border border-line shadow-sm space-y-1"
                            >
                                <label
                                    class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1"
                                >
                                    <Users size={12} />
                                    Servings
                                </label>
                                <input
                                    type="number"
                                    bind:value={editServings}
                                    class="w-full bg-transparent border-none p-0 text-sm font-bold focus:ring-0"
                                />
                            </div>
                            <div
                                class="bg-surface p-3 rounded-xl border border-line shadow-sm space-y-1"
                            >
                                <label
                                    class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1"
                                >
                                    <Clock size={12} />
                                    Prep (m)
                                </label>
                                <input
                                    type="number"
                                    bind:value={editPrepTime}
                                    class="w-full bg-transparent border-none p-0 text-sm font-bold focus:ring-0"
                                />
                            </div>
                            <div
                                class="bg-surface p-3 rounded-xl border border-line shadow-sm space-y-1"
                            >
                                <label
                                    class="text-[10px] font-bold text-foreground-subtle uppercase tracking-widest flex items-center gap-1"
                                >
                                    <ChefHat size={12} />
                                    Cook (m)
                                </label>
                                <input
                                    type="number"
                                    bind:value={editCookTime}
                                    class="w-full bg-transparent border-none p-0 text-sm font-bold focus:ring-0"
                                />
                            </div>
                        </div>
                    {:else}
                        {#if recipe.servings}
                            <div class="flex items-center gap-1.5">
                                <Users size={18} class="text-accent" />
                                <span>{recipe.servings} servings</span>
                            </div>
                        {/if}
                        {#if recipe.prep_time}
                            <div class="flex items-center gap-1.5">
                                <Clock size={18} class="text-accent" />
                                <span>Prep: {formatTime(recipe.prep_time)}</span
                                >
                            </div>
                        {/if}
                        {#if recipe.cook_time}
                            <div class="flex items-center gap-1.5">
                                <ChefHat size={18} class="text-accent" />
                                <span>Cook: {formatTime(recipe.cook_time)}</span
                                >
                            </div>
                        {/if}
                    {/if}
                </div>

                {#if isEditing}
                    <div class="space-y-4">
                        <div
                            class="flex items-center gap-2 text-foreground-muted font-bold text-sm uppercase tracking-wider"
                        >
                            <TagIcon size={16} />
                            <span>Tags</span>
                        </div>
                        <div
                            class="bg-surface p-4 rounded-xl border border-line shadow-sm space-y-4"
                        >
                            <div class="flex flex-wrap gap-2">
                                {#each availableTags as tag}
                                    <button
                                        onclick={() => toggleTag(tag.id)}
                                        class="px-3 py-1 rounded-full text-xs font-medium border transition {editTagIds.includes(
                                            tag.id,
                                        )
                                            ? 'bg-accent text-background border-accent'
                                            : 'bg-surface-sunken text-foreground-muted border-line hover:border-accent/50'}"
                                    >
                                        {tag.name}
                                    </button>
                                {/each}
                            </div>
                            <div class="flex gap-2">
                                <input
                                    type="text"
                                    bind:value={newTagName}
                                    placeholder="New tag name..."
                                    class="flex-1 bg-surface-sunken border border-line rounded-lg px-3 py-1.5 text-sm outline-none"
                                    onkeydown={(e) =>
                                        e.key === "Enter" && handleCreateTag()}
                                />
                                <button
                                    onclick={handleCreateTag}
                                    class="px-3 py-1.5 bg-surface-sunken text-foreground-muted rounded-lg text-sm font-bold hover:bg-surface-raised transition"
                                >
                                    Add
                                </button>
                            </div>
                        </div>
                    </div>
                {:else if recipe.tags.length > 0}
                    <div class="flex flex-wrap gap-2">
                        {#each recipe.tags as tag}
                            <span
                                class="px-3 py-1 bg-accent/10 text-accent rounded-full text-xs font-bold"
                            >
                                {tag.name}
                            </span>
                        {/each}
                    </div>
                {/if}
            </section>

            <!-- Ingredients -->
            <section class="space-y-4">
                <div class="flex items-center justify-between">
                    <h3 class="text-lg font-bold flex items-center gap-2">
                        <ChefHat size={20} class="text-accent" />
                        Ingredients
                    </h3>
                    {#if isEditing}
                        <button
                            onclick={addIngredient}
                            class="text-accent text-sm font-bold flex items-center gap-1 hover:bg-accent/5 px-2 py-1 rounded transition"
                        >
                            <Plus size={16} />
                            Add
                        </button>
                    {/if}
                </div>

                {#if isEditing}
                    <div class="space-y-3">
                        {#each editIngredients as ing, i}
                            <div
                                class="flex gap-2 items-start bg-surface p-3 rounded-xl border border-line shadow-sm"
                            >
                                <div class="flex-1 space-y-2">
                                    <select
                                        bind:value={ing.ingredient_id}
                                        onchange={(e) =>
                                            handleIngredientChange(i, e)}
                                        class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none"
                                    >
                                        <option value=""
                                            >Select Ingredient</option
                                        >
                                        <option
                                            value="new"
                                            class="text-accent font-bold"
                                            >+ Add New Ingredient</option
                                        >
                                        {#each availableIngredients as a}
                                            <option value={a.id}
                                                >{a.name}</option
                                            >
                                        {/each}
                                    </select>
                                    <div class="flex gap-2">
                                        <input
                                            type="number"
                                            step="any"
                                            bind:value={ing.quantity}
                                            placeholder="Qty"
                                            class="w-20 bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none"
                                        />
                                        <select
                                            bind:value={ing.unit}
                                            class="flex-1 bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none"
                                        >
                                            {#each COMMON_UNITS as u}
                                                <option value={u}>{u}</option>
                                            {/each}
                                        </select>
                                        <label
                                            class="flex items-center gap-1 px-2 whitespace-nowrap"
                                        >
                                            <input
                                                type="checkbox"
                                                bind:checked={ing.is_optional}
                                                class="w-3 h-3 rounded"
                                            />
                                            <span
                                                class="text-xs text-foreground-muted"
                                                >Opt.</span
                                            >
                                        </label>
                                    </div>
                                </div>
                                <button
                                    onclick={() => removeIngredient(i)}
                                    class="p-2 text-foreground-subtle hover:text-danger transition"
                                >
                                    <Trash2 size={18} />
                                </button>
                            </div>
                        {/each}
                    </div>
                {:else}
                    <div
                        class="bg-surface rounded-2xl border border-line overflow-hidden shadow-sm"
                    >
                        <ul class="divide-y divide-line">
                            {#each recipe.ingredients as ri}
                                <li
                                    class="p-4 flex justify-between items-center hover:bg-surface-raised transition"
                                >
                                    <div class="flex flex-col">
                                        <span
                                            class="font-medium {ri.is_optional
                                                ? 'text-foreground-subtle'
                                                : ''}"
                                        >
                                            {ri.ingredient.name}
                                            {#if ri.is_optional}
                                                <span
                                                    class="text-xs font-normal ml-1"
                                                    >(optional)</span
                                                >
                                            {/if}
                                        </span>
                                    </div>
                                    <span
                                        class="text-sm font-bold bg-surface-sunken px-2 py-1 rounded-lg"
                                    >
                                        {ri.quantity}
                                        {ri.unit}
                                    </span>
                                </li>
                            {/each}
                            {#each recipe.components as comp}
                                <li
                                    class="p-4 flex justify-between items-center hover:bg-surface-raised transition"
                                >
                                    <a
                                        href="/recipes/{comp.child.id}"
                                        class="font-medium text-accent hover:underline"
                                    >
                                        {comp.child.title} (sub-recipe)
                                    </a>
                                    <span
                                        class="text-sm font-bold bg-accent/10 text-accent px-2 py-1 rounded-lg"
                                    >
                                        {comp.servings_needed} servings
                                    </span>
                                </li>
                            {/each}
                        </ul>
                    </div>
                {/if}

                <!-- Sub-recipes editing -->
                {#if isEditing}
                    <div class="space-y-4 pt-4">
                        <div class="flex items-center justify-between">
                            <h4
                                class="text-sm font-bold text-foreground-muted uppercase tracking-wider"
                            >
                                Sub-recipes
                            </h4>
                            <button
                                onclick={addComponent}
                                class="text-accent text-sm font-bold flex items-center gap-1 hover:bg-accent/5 px-2 py-1 rounded transition"
                            >
                                <Plus size={16} />
                                Add Sub
                            </button>
                        </div>
                        <div class="space-y-3">
                            {#each editComponents as comp, i}
                                <div
                                    class="flex gap-2 items-start bg-surface p-3 rounded-xl border border-line shadow-sm"
                                >
                                    <div class="flex-1 space-y-2">
                                        <select
                                            bind:value={comp.child_id}
                                            class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none"
                                        >
                                            <option value=""
                                                >Select Recipe</option
                                            >
                                            {#each availableRecipes as r}
                                                <option value={r.id}
                                                    >{r.title}</option
                                                >
                                            {/each}
                                        </select>
                                        <div class="flex items-center gap-2">
                                            <label
                                                class="text-xs text-foreground-muted"
                                                >Need</label
                                            >
                                            <input
                                                type="number"
                                                step="any"
                                                bind:value={
                                                    comp.servings_needed
                                                }
                                                class="w-20 bg-surface-sunken border border-line rounded-lg px-3 py-1 text-sm outline-none"
                                            />
                                            <label
                                                class="text-xs text-foreground-muted"
                                                >servings</label
                                            >
                                        </div>
                                    </div>
                                    <button
                                        onclick={() => removeComponent(i)}
                                        class="p-2 text-foreground-subtle hover:text-danger transition"
                                    >
                                        <Trash2 size={18} />
                                    </button>
                                </div>
                            {/each}
                        </div>
                    </div>
                {/if}
            </section>

            <!-- Steps -->
            <section class="space-y-4 pb-12">
                <div class="flex items-center justify-between">
                    <h3 class="text-lg font-bold flex items-center gap-2">
                        <Clock size={20} class="text-accent" />
                        Steps
                    </h3>
                    {#if isEditing && editSteps.length === 0}
                        <button
                            onclick={() => addStep()}
                            class="text-accent text-sm font-bold flex items-center gap-1 hover:bg-accent/5 px-2 py-1 rounded transition"
                        >
                            <Plus size={16} />
                            Add First Step
                        </button>
                    {/if}
                </div>

                <div class="space-y-0">
                    {#each isEditing ? editSteps : recipe.steps as step, i}
                        <div class="flex gap-4">
                            <div class="flex flex-col items-center gap-2">
                                <div
                                    class="w-8 h-8 rounded-full bg-accent text-background flex items-center justify-center font-bold shrink-0"
                                >
                                    {i + 1}
                                </div>
                                {#if i < (isEditing ? editSteps.length : recipe.steps.length) - 1 || isEditing}
                                    <div class="w-0.5 flex-1 bg-line"></div>
                                {/if}
                            </div>
                            <div class="flex-1 pb-4">
                                <div
                                    class="bg-surface p-4 rounded-2xl border border-line shadow-sm space-y-3"
                                >
                                    {#if isEditing}
                                        <div
                                            class="flex justify-between items-center gap-2"
                                        >
                                            <div
                                                class="flex bg-surface-sunken rounded-lg p-1"
                                            >
                                                <button
                                                    onclick={() =>
                                                        (step.step_type =
                                                            "prep")}
                                                    class="px-3 py-1 text-[10px] font-bold uppercase rounded-md transition {step.step_type ===
                                                    'prep'
                                                        ? 'bg-surface text-foreground shadow-sm'
                                                        : 'text-foreground-subtle'}"
                                                    >Prep</button
                                                >
                                                <button
                                                    onclick={() =>
                                                        (step.step_type =
                                                            "cook")}
                                                    class="px-3 py-1 text-[10px] font-bold uppercase rounded-md transition {step.step_type ===
                                                    'cook'
                                                        ? 'bg-surface text-foreground shadow-sm'
                                                        : 'text-foreground-subtle'}"
                                                    >Cook</button
                                                >
                                            </div>
                                            <div
                                                class="flex items-center gap-2"
                                            >
                                                <div
                                                    class="flex items-center bg-surface-sunken rounded-lg px-2 py-1"
                                                >
                                                    <Clock
                                                        size={12}
                                                        class="text-foreground-subtle mr-1"
                                                    />
                                                    <input
                                                        type="number"
                                                        bind:value={
                                                            step.duration_min
                                                        }
                                                        placeholder="0"
                                                        class="w-10 bg-transparent border-none p-0 text-xs font-bold focus:ring-0"
                                                    />
                                                    <span
                                                        class="text-[10px] font-bold text-foreground-subtle"
                                                        >m</span
                                                    >
                                                </div>
                                                <button
                                                    onclick={() =>
                                                        moveStep(i, "up")}
                                                    disabled={i === 0}
                                                    class="p-1 text-foreground-subtle disabled:opacity-30"
                                                    ><ChevronLeft
                                                        size={16}
                                                        class="rotate-90"
                                                    /></button
                                                >
                                                <button
                                                    onclick={() =>
                                                        moveStep(i, "down")}
                                                    disabled={i ===
                                                        editSteps.length - 1}
                                                    class="p-1 text-foreground-subtle disabled:opacity-30"
                                                    ><ChevronLeft
                                                        size={16}
                                                        class="-rotate-90"
                                                    /></button
                                                >
                                                <button
                                                    onclick={() =>
                                                        removeStep(i)}
                                                    class="p-1 text-foreground-subtle hover:text-danger"
                                                    ><X size={16} /></button
                                                >
                                            </div>
                                        </div>
                                        <textarea
                                            bind:value={step.description}
                                            placeholder="What to do in this step?"
                                            rows="2"
                                            class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-accent/20"
                                        ></textarea>
                                    {:else}
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
                                    {/if}
                                </div>
                            </div>
                        </div>

                        {#if isEditing}
                            <div class="flex gap-4">
                                <div class="w-8 flex flex-col items-center">
                                    <div class="w-0.5 flex-1 bg-line"></div>
                                </div>
                                <div class="flex-1 py-2">
                                    <button
                                        onclick={() => addStep(i)}
                                        class="w-full py-2 border-2 border-dashed border-line rounded-xl flex items-center justify-center text-foreground-subtle hover:text-accent hover:border-accent/30 hover:bg-accent/5 transition-all group"
                                    >
                                        <Plus
                                            size={16}
                                            class="group-hover:scale-125 transition-transform mr-2"
                                        />
                                        <span
                                            class="text-xs font-bold uppercase tracking-widest"
                                            >Next Step</span
                                        >
                                    </button>
                                </div>
                            </div>
                        {/if}
                    {/each}
                </div>
            </section>
        </div>
    {/if}
</div>

<ImageModal
    src={previewImage}
    alt={recipe?.title}
    onClose={() => (previewImage = null)}
/>

<QuickAddIngredient bind:open={quickAddOpen} onCreated={onIngredientCreated} />

<style>
    input[type="number"]::-webkit-inner-spin-button,
    input[type="number"]::-webkit-outer-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }
</style>
