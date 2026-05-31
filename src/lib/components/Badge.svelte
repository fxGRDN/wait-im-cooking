<script lang="ts">
    import { type Component } from "svelte";
    import { type LucideProps } from "@lucide/svelte";

    let {
        label,
        icon: Icon = null,
        variant = "default",
        class: className = "",
    } = $props<{
        label: string;
        icon?: Component<LucideProps> | null;
        variant?: "default" | "success" | "danger" | "muted" | "accent";
        class?: string;
    }>();

    const variants: Record<string, string> = {
        default: "bg-surface-sunken text-foreground-subtle border border-line",
        success: "bg-success text-background shadow-sm",
        danger: "bg-danger/10 text-danger border border-danger/20",
        muted: "bg-surface-sunken text-foreground-muted",
        accent: "bg-accent/10 text-accent border border-accent/20",
    };
</script>

<span
    class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded-md text-[9px] font-black uppercase tracking-tight {variants[
        variant
    ]} {className}"
>
    {#if Icon}
        <Icon size={10} strokeWidth={3} />
    {/if}
    {label}
</span>
