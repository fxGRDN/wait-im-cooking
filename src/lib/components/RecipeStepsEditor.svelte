<script lang="ts">
    import { Clock, Plus, ChevronLeft, X } from "@lucide/svelte";
    import type { StepInput } from "$lib/types";

    let {
        steps = $bindable([]),
    }: {
        steps: StepInput[];
    } = $props();

    function addStep(index?: number) {
        const newStep: StepInput = {
            step_order: 0,
            step_type: "prep",
            description: "",
            duration_min: 0,
        };

        if (typeof index === "number") {
            steps.splice(index + 1, 0, newStep);
        } else {
            steps.push(newStep);
        }

        // Fix step orders
        updateOrders();
    }

    function removeStep(index: number) {
        steps.splice(index, 1);
        updateOrders();
    }

    function moveStep(index: number, direction: "up" | "down") {
        if (direction === "up" && index > 0) {
            const temp = steps[index];
            steps[index] = steps[index - 1];
            steps[index - 1] = temp;
        } else if (direction === "down" && index < steps.length - 1) {
            const temp = steps[index];
            steps[index] = steps[index + 1];
            steps[index + 1] = temp;
        }
        updateOrders();
    }

    function updateOrders() {
        steps.forEach((s, i) => (s.step_order = i + 1));
    }
</script>

<section class="space-y-4 pb-12">
    <div class="flex items-center justify-between">
        <h3 class="text-lg font-bold flex items-center gap-2">
            <Clock size={20} class="text-accent" />
            Steps
        </h3>
        {#if steps.length === 0}
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
        {#each steps as step, i}
            <div class="flex gap-4">
                <div class="flex flex-col items-center gap-2">
                    <div
                        class="w-8 h-8 rounded-full bg-accent text-background flex items-center justify-center font-bold shrink-0"
                    >
                        {i + 1}
                    </div>
                    {#if i < steps.length - 1}
                        <div class="w-0.5 flex-1 bg-line"></div>
                    {/if}
                </div>
                <div class="flex-1 pb-4">
                    <div
                        class="bg-surface p-4 rounded-2xl border border-line shadow-sm space-y-3"
                    >
                        <div class="flex justify-between items-center gap-2">
                            <div class="flex bg-surface-sunken rounded-lg p-1">
                                <button
                                    onclick={() => (step.step_type = "prep")}
                                    class="px-3 py-1 text-[10px] font-bold uppercase rounded-md transition {step.step_type ===
                                    'prep'
                                        ? 'bg-surface text-foreground shadow-sm'
                                        : 'text-foreground-subtle'}"
                                >
                                    Prep
                                </button>
                                <button
                                    onclick={() => (step.step_type = "cook")}
                                    class="px-3 py-1 text-[10px] font-bold uppercase rounded-md transition {step.step_type ===
                                    'cook'
                                        ? 'bg-surface text-foreground shadow-sm'
                                        : 'text-foreground-subtle'}"
                                >
                                    Cook
                                </button>
                            </div>
                            <div class="flex items-center gap-2">
                                <div
                                    class="flex items-center bg-surface-sunken rounded-lg px-2 py-1"
                                >
                                    <Clock
                                        size={12}
                                        class="text-foreground-subtle mr-1"
                                    />
                                    <input
                                        type="number"
                                        bind:value={step.duration_min}
                                        placeholder="0"
                                        class="w-10 bg-transparent border-none p-0 text-xs font-bold focus:ring-0"
                                    />
                                    <span
                                        class="text-[10px] font-bold text-foreground-subtle"
                                        >m</span
                                    >
                                </div>
                                <button
                                    onclick={() => moveStep(i, "up")}
                                    disabled={i === 0}
                                    class="p-1 text-foreground-subtle disabled:opacity-30"
                                >
                                    <ChevronLeft size={16} class="rotate-90" />
                                </button>
                                <button
                                    onclick={() => moveStep(i, "down")}
                                    disabled={i === steps.length - 1}
                                    class="p-1 text-foreground-subtle disabled:opacity-30"
                                >
                                    <ChevronLeft size={16} class="-rotate-90" />
                                </button>
                                <button
                                    onclick={() => removeStep(i)}
                                    class="p-1 text-foreground-subtle hover:text-danger"
                                >
                                    <X size={16} />
                                </button>
                            </div>
                        </div>
                        <textarea
                            bind:value={step.description}
                            placeholder="What to do in this step?"
                            rows="2"
                            class="w-full bg-surface-sunken border border-line rounded-lg px-3 py-2 text-sm outline-none focus:ring-1 focus:ring-accent/20"
                        ></textarea>

                        <button
                            onclick={() => addStep(i)}
                            class="w-full py-2 border-2 border-dashed border-line rounded-xl text-foreground-subtle text-[10px] font-bold uppercase tracking-widest hover:bg-surface-sunken transition flex items-center justify-center gap-1"
                        >
                            <Plus size={14} />
                            Insert Step
                        </button>
                    </div>
                </div>
            </div>
        {/each}
    </div>
</section>
