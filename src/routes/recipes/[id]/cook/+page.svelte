<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { page } from "$app/stores";
    import { getRecipeWithTree } from "$lib/services/recipes";
    import type { RecipeWithTree, Step } from "$lib/types";
    import {
        ChevronLeft,
        Play,
        Pause,
        RotateCcw,
        SkipForward,
        CheckCircle2,
        Clock,
        ChefHat,
        Check,
        Square,
        CheckSquare,
        ChevronDown,
        ChevronUp,
    } from "lucide-svelte";
    import { goto } from "$app/navigation";
    import { createCookLog } from "$lib/services/cooklog";

    let recipe = $state<RecipeWithTree | null>(null);
    let loading = $state(true);

    // UI State
    let completedSteps = $state<Set<string>>(new Set());
    let checkedIngredients = $state<Set<string>>(new Set());
    let showIngredients = $state(true);

    // Timer state - attached to a specific step
    let activeTimerStepId = $state<string | null>(null);
    let timeLeft = $state(0);
    let timerRunning = $state(false);
    let timerInterval: number | null = null;

    const id = $page.params.id;

    onMount(async () => {
        try {
            recipe = await getRecipeWithTree(id);
        } catch (e) {
            console.error(e);
        } finally {
            loading = false;
        }

        window.addEventListener("complete-cooking", handleComplete);
        return () =>
            window.removeEventListener("complete-cooking", handleComplete);
    });

    $effect(() => {
        if (recipe) {
            const isCompleted = completedSteps.size === recipe.steps.length;
            window.dispatchEvent(
                new CustomEvent("cook-status", {
                    detail: { completed: isCompleted },
                }),
            );
        }
    });

    async function handleComplete() {
        if (!recipe) return;
        try {
            const historyId = await createCookLog(
                {
                    recipe_id: recipe.id,
                    servings_made: recipe.servings || 1,
                    duration_min:
                        (recipe.prep_time || 0) + (recipe.cook_time || 0),
                    rating: 5,
                    notes: null,
                },
                [],
                true,
            );
            // Redirect to the newly created history entry in edit mode and replace the cooking page in history
            goto(`/recipes/history/${historyId}?edit=true`, {
                replaceState: true,
            });
        } catch (e) {
            console.error(e);
            alert("Failed to log cooking session.");
        }
    }

    onDestroy(() => {
        stopTimer();
    });

    function toggleIngredient(id: string) {
        if (checkedIngredients.has(id)) {
            checkedIngredients.delete(id);
        } else {
            checkedIngredients.add(id);
        }
        checkedIngredients = new Set(checkedIngredients);
    }

    function toggleStep(id: string) {
        if (completedSteps.has(id)) {
            completedSteps.delete(id);
        } else {
            completedSteps.add(id);
        }
        completedSteps = new Set(completedSteps);
    }

    function toggleTimer() {
        if (timerRunning) {
            stopTimer();
        } else {
            if (activeTimerStepId) {
                startTimerById(activeTimerStepId);
            }
        }
    }

    function startTimerById(stepId: string) {
        if (timeLeft <= 0) return;
        timerRunning = true;
        timerInterval = window.setInterval(() => {
            timeLeft -= 1;
            if (timeLeft <= 0) {
                stopTimer();
                if ("vibrate" in navigator) navigator.vibrate([500, 200, 500]);
            }
        }, 1000);
    }

    function startTimer(step: Step) {
        if (activeTimerStepId === step.id && timerRunning) {
            stopTimer();
            return;
        }

        if (activeTimerStepId !== step.id) {
            stopTimer();
            activeTimerStepId = step.id;
            timeLeft = (step.duration_min || 0) * 60;
        }

        startTimerById(step.id);
    }

    function stopTimer() {
        timerRunning = false;
        if (timerInterval) {
            clearInterval(timerInterval);
            timerInterval = null;
        }
    }

    function resetTimer(step: Step) {
        stopTimer();
        activeTimerStepId = step.id;
        timeLeft = (step.duration_min || 0) * 60;
    }

    function formatTime(seconds: number) {
        const m = Math.floor(seconds / 60);
        const s = seconds % 60;
        return `${m}:${s.toString().padStart(2, "0")}`;
    }

    let progress = $derived(
        recipe ? (completedSteps.size / recipe.steps.length) * 100 : 0,
    );

    // A step is "unlockable" if all previous steps are completed
    function isStepUnlocked(index: number) {
        if (!recipe) return false;
        if (index === 0) return true;
        for (let i = 0; i < index; i++) {
            if (!completedSteps.has(recipe.steps[i].id)) return false;
        }
        return true;
    }
</script>

<div class="min-h-screen bg-surface text-foreground flex flex-col pb-20">
    <!-- Header -->
    <div
        class="border-b border-line px-4 pt-[calc(1rem+env(safe-area-inset-top))] pb-4 bg-surface sticky top-0 z-20 flex flex-col shadow-sm"
    >
        <div class="flex justify-between items-center w-full">
            <button
                onclick={() => history.back()}
                class="p-2 -ml-2 hover:bg-surface-sunken rounded-full transition"
            >
                <ChevronLeft size={24} />
            </button>
            <div class="text-center flex-1 px-4 truncate">
                <h1 class="text-sm font-bold truncate">{recipe?.title}</h1>
                <p
                    class="text-[10px] text-foreground-muted uppercase tracking-widest font-bold"
                >
                    {completedSteps.size} of {recipe?.steps.length || 0} Steps Done
                </p>
            </div>
            <div class="w-10"></div>
        </div>

        <!-- Progress Bar (Internal) -->
        <div
            class="h-1 bg-surface-sunken w-full mt-2 rounded-full overflow-hidden"
        >
            <div
                class="h-full bg-accent transition-all duration-500"
                style="width: {progress}%"
            ></div>
        </div>
    </div>

    {#if loading}
        <div class="flex-1 flex items-center justify-center">
            <div
                class="animate-spin rounded-full h-12 w-12 border-4 border-accent border-t-transparent"
            ></div>
        </div>
    {:else if recipe}
        <main class="flex-1 p-4 max-w-2xl mx-auto w-full space-y-8">
            <!-- Ingredients Section -->
            <section class="space-y-3">
                <button
                    onclick={() => (showIngredients = !showIngredients)}
                    class="w-full flex items-center justify-between text-sm font-bold text-foreground-subtle uppercase tracking-widest"
                >
                    <div class="flex items-center gap-2">
                        <ChefHat size={16} />
                        Ingredients ({checkedIngredients.size}/{recipe
                            .ingredients.length})
                    </div>
                    {#if showIngredients}
                        <ChevronUp size={16} />
                    {:else}
                        <ChevronDown size={16} />
                    {/if}
                </button>

                {#if showIngredients}
                    <div
                        class="bg-surface rounded-2xl border border-line shadow-sm overflow-hidden animate-in fade-in slide-in-from-top-2 duration-200"
                    >
                        <div class="grid grid-cols-1 divide-y divide-line">
                            {#each recipe.ingredients as ing}
                                <button
                                    onclick={() =>
                                        toggleIngredient(ing.ingredient_id)}
                                    class="flex items-center gap-3 p-4 text-left hover:bg-surface-raised transition"
                                >
                                    {#if checkedIngredients.has(ing.ingredient_id)}
                                        <CheckSquare
                                            size={20}
                                            class="text-accent"
                                        />
                                    {:else}
                                        <Square
                                            size={20}
                                            class="text-foreground-subtle"
                                        />
                                    {/if}
                                    <div
                                        class="flex-1 flex justify-between items-center"
                                    >
                                        <span
                                            class="text-sm font-medium {checkedIngredients.has(
                                                ing.ingredient_id,
                                            )
                                                ? 'text-foreground-subtle line-through'
                                                : ''}"
                                        >
                                            {ing.ingredient.name}
                                        </span>
                                        <span
                                            class="text-xs font-bold text-foreground-subtle bg-surface-sunken px-2 py-0.5 rounded-md"
                                        >
                                            {ing.quantity}
                                            {ing.unit}
                                        </span>
                                    </div>
                                </button>
                            {/each}
                        </div>
                    </div>
                {/if}
            </section>

            <!-- Steps Section -->
            <section class="space-y-4">
                <div
                    class="flex items-center gap-2 text-sm font-bold text-foreground-subtle uppercase tracking-widest"
                >
                    <Clock size={16} />
                    Steps
                </div>

                <div class="space-y-4">
                    {#each recipe.steps as step, i}
                        {@const unlocked = isStepUnlocked(i)}
                        {@const completed = completedSteps.has(step.id)}
                        {@const isTimerActive = activeTimerStepId === step.id}

                        <div
                            class="relative flex gap-4 transition-opacity duration-300 {unlocked
                                ? 'opacity-100'
                                : 'opacity-40 cursor-not-allowed'}"
                        >
                            <!-- Vertical Line and Bubble -->
                            <div class="flex flex-col items-center">
                                <button
                                    onclick={() =>
                                        unlocked && toggleStep(step.id)}
                                    disabled={!unlocked}
                                    class="w-10 h-10 rounded-full flex items-center justify-center border-2 transition z-10
                                        {completed
                                        ? 'bg-success border-success text-white'
                                        : 'bg-surface border-line text-foreground-subtle hover:border-accent hover:text-accent'}"
                                >
                                    {#if completed}
                                        <Check size={24} strokeWidth={3} />
                                    {:else}
                                        <span class="text-sm font-bold"
                                            >{i + 1}</span
                                        >
                                    {/if}
                                </button>
                                {#if i < recipe.steps.length - 1}
                                    <div
                                        class="w-0.5 flex-1 bg-line -mt-1 -mb-1"
                                    ></div>
                                {/if}
                            </div>

                            <!-- Step Card -->
                            <div class="flex-1 pb-8">
                                <div
                                    class="bg-surface p-5 rounded-2xl border shadow-sm space-y-4 transition-all
                                        {completed
                                        ? 'border-success-edge bg-success-soft/20'
                                        : 'border-line'}
                                        {unlocked ? '' : 'pointer-events-none'}"
                                >
                                    <div
                                        class="flex justify-between items-start"
                                    >
                                        <span
                                            class="text-[10px] font-bold uppercase tracking-wider text-foreground-subtle"
                                        >
                                            {step.step_type}
                                        </span>

                                        <!-- Mini Timer Integrated into Step -->
                                        {#if step.duration_min}
                                            <div
                                                class="flex items-center gap-2"
                                            >
                                                {#if isTimerActive}
                                                    <div
                                                        class="flex items-center gap-1.5 bg-accent/10 px-2 py-1 rounded-lg"
                                                    >
                                                        <span
                                                            class="text-xs font-mono font-bold text-accent"
                                                            >{formatTime(
                                                                timeLeft,
                                                            )}</span
                                                        >
                                                        <button
                                                            onclick={() =>
                                                                toggleTimer()}
                                                            class="text-accent hover:opacity-70"
                                                        >
                                                            {#if timerRunning}
                                                                <Pause
                                                                    size={14}
                                                                    fill="currentColor"
                                                                />
                                                            {:else}
                                                                <Play
                                                                    size={14}
                                                                    fill="currentColor"
                                                                />
                                                            {/if}
                                                        </button>
                                                        <button
                                                            onclick={() =>
                                                                resetTimer(
                                                                    step,
                                                                )}
                                                            class="text-accent/50 hover:text-accent"
                                                        >
                                                            <RotateCcw
                                                                size={14}
                                                            />
                                                        </button>
                                                        <button
                                                            onclick={() => {
                                                                stopTimer();
                                                                timeLeft = 0;
                                                            }}
                                                            class="text-accent/50 hover:text-accent"
                                                            title="Skip"
                                                        >
                                                            <SkipForward
                                                                size={14}
                                                            />
                                                        </button>
                                                    </div>
                                                {:else}
                                                    <button
                                                        onclick={() =>
                                                            startTimer(step)}
                                                        class="flex items-center gap-1.5 bg-surface-sunken hover:bg-accent/10 text-foreground-subtle hover:text-accent px-2 py-1 rounded-lg transition"
                                                    >
                                                        <Clock size={14} />
                                                        <span
                                                            class="text-xs font-bold"
                                                            >{step.duration_min}m</span
                                                        >
                                                        <Play
                                                            size={12}
                                                            fill="currentColor"
                                                        />
                                                    </button>
                                                {/if}
                                            </div>
                                        {/if}
                                    </div>

                                    <p
                                        class="text-sm font-medium leading-relaxed {completed
                                            ? 'text-foreground-subtle'
                                            : 'text-foreground'}"
                                    >
                                        {step.description}
                                    </p>

                                    <!-- Next/Done Action Button -->
                                    {#if unlocked && !completed}
                                        <button
                                            onclick={() => toggleStep(step.id)}
                                            class="w-full flex items-center justify-center gap-2 py-3 bg-accent text-background rounded-xl font-bold shadow-md hover:opacity-90 transition active:scale-[0.98]"
                                        >
                                            <Check size={18} strokeWidth={3} />
                                            Done
                                        </button>
                                    {/if}
                                </div>
                            </div>
                        </div>
                    {/each}
                </div>
            </section>
        </main>
    {/if}
</div>

<style>
    /* Prevent selection of step numbers */
    span,
    button {
        user-select: none;
    }
</style>
