<script lang="ts">
  import { fcd, type WithElementRef } from "$lib/utils.js";
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  let {
    ref = $bindable(null),
    class: className,
    showOnHover = false,
    children,
    child,
    ...restProps
  }: WithElementRef<HTMLButtonAttributes> & {
    child?: Snippet<[{ props: Record<string, unknown> }]>;
    showOnHover?: boolean;
  } = $props();

  const mergedProps = $derived({
    class: fcd(
      {
        "hover": [
          "hover:bg-sidebar-accent",
          "hover:text-sidebar-accent-foreground",
          "peer-hover/menu-button:text-sidebar-accent-foreground",
        ],
        "border": [
          "rounded-md",
          "ring-sidebar-ring",
          "focus-visible:ring-2",
          "outline-hidden",
        ],
        "svg": [
          "[&>svg]:shrink-0",
          "[&>svg]:size-4",
        ],
        "layout": [
          "flex",
          "aspect-square",
          "w-5",
          "items-center",
          "justify-center",
          "p-0",
        ],
        "transition": "transition-transform",
        "positioning": [
          "absolute",
          "right-1",
          "top-1.5",
        ],
        "after": [
          "after:absolute",
          "after:-inset-2",
        ],
        "md": "md:after:hidden",
        "peer-data": [
          "peer-data-[size=sm]/menu-button:top-1",
          "peer-data-[size=default]/menu-button:top-1.5",
          "peer-data-[size=lg]/menu-button:top-2.5",
        ],
        "collapsible": "group-data-[collapsible=icon]:hidden",
        "text": "text-sidebar-foreground",
      },
      showOnHover &&
        "peer-data-[active=true]/menu-button:text-sidebar-accent-foreground group-focus-within/menu-item:opacity-100 group-hover/menu-item:opacity-100 data-[state=open]:opacity-100 md:opacity-0",
      className,
    ),

    "data-slot": "sidebar-menu-action",
    "data-sidebar": "menu-action",
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
