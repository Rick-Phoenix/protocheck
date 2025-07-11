<script lang="ts">
  import { fcd, type WithElementRef } from "$lib/utils.js";
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  let {
    ref = $bindable(null),
    class: className,
    children,
    child,
    ...restProps
  }: WithElementRef<HTMLButtonAttributes> & {
    child?: Snippet<[{ props: Record<string, unknown> }]>;
  } = $props();

  const mergedProps = $derived({
    class: fcd({
      "border": [
        "ring-sidebar-ring",
        "rounded-md",
        "focus-visible:ring-2",
        "outline-hidden",
      ],
      "svg": [
        "[&>svg]:shrink-0",
        "[&>svg]:size-4",
      ],
      "hover": [
        "hover:bg-sidebar-accent",
        "hover:text-sidebar-accent-foreground",
      ],
      "positioning": [
        "absolute",
        "right-3",
        "top-3.5",
      ],
      "layout": [
        "flex",
        "aspect-square",
        "w-5",
        "items-center",
        "justify-center",
        "p-0",
      ],
      "collapsible": "group-data-[collapsible=icon]:hidden",
      "text": "text-sidebar-foreground",
      "transition": "transition-transform",
      "after": [
        "after:absolute",
        "after:-inset-2",
      ],
      "md": "md:after:hidden",
    }, className),

    "data-slot": "sidebar-group-action",
    "data-sidebar": "group-action",
    ...restProps,
  });
</script>

{#if child}
  {@render child({ props: mergedProps })}
{:else}
  <button bind:this={ref} {...mergedProps}>
    {@render children?.()}
  </button>
{/if}
