<script lang="ts">
  import type {
    HTMLInputAttributes,
    HTMLInputTypeAttribute,
  } from "svelte/elements";
  import { cn, fcd, type WithElementRef } from "$lib/utils.js";

  type InputType = Exclude<HTMLInputTypeAttribute, "file">;

  type Props = WithElementRef<
    & Omit<HTMLInputAttributes, "type">
    & ({ type: "file"; files?: FileList } | {
      type?: InputType;
      files?: undefined;
    })
  >;

  let {
    ref = $bindable(null),
    value = $bindable(),
    type,
    files = $bindable(),
    class: className,
    ...restProps
  }: Props = $props();
</script>

{#if type === "file"}
  <input
    bind:this={ref}
    data-slot="input"
    class={fcd({
      "bg": [
        "selection:bg-primary",
        "dark:bg-input/30",
        "bg-transparent",
      ],
      "text": [
        "selection:text-primary-foreground",
        "text-sm",
        "font-medium",
      ],
      "border": [
        "border-input",
        "ring-offset-background",
        "shadow-xs",
        "rounded-md",
        "border",
        "outline-none",
        "focus-visible:border-ring",
        "focus-visible:ring-ring/50",
        "focus-visible:ring-[3px]",
      ],
      "disabled": [
        "disabled:cursor-not-allowed",
        "disabled:opacity-50",
      ],
      "invalid": [
        "aria-invalid:ring-destructive/20",
        "dark:aria-invalid:ring-destructive/40",
        "aria-invalid:border-destructive",
      ],
      "placeholder": "placeholder:text-muted-foreground",
      "layout": [
        "flex",
        "h-9",
        "w-full",
        "min-w-0",
        "px-3",
        "pt-1.5",
      ],
      "transition": "transition-[color,box-shadow]",
    }, className)}
    type="file"
    bind:files
    bind:value
    {...restProps}
  />
{:else}
  <input
    bind:this={ref}
    data-slot="input"
    class={cn(
      "border-input bg-background selection:bg-primary dark:bg-input/30 selection:text-primary-foreground ring-offset-background placeholder:text-muted-foreground shadow-xs flex h-9 w-full min-w-0 rounded-md border px-3 py-1 text-base outline-none transition-[color,box-shadow] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
      "focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
      "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
      className,
    )}
    {type}
    bind:value
    {...restProps}
  />
{/if}
