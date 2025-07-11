<script lang="ts">
  import { fcd, type WithElementRef } from "$lib/utils.js";
  import type { Snippet } from "svelte";
  import type { HTMLAnchorAttributes } from "svelte/elements";

  let {
    ref = $bindable(null),
    children,
    child,
    class: className,
    size = "md",
    isActive = false,
    ...restProps
  }: WithElementRef<HTMLAnchorAttributes> & {
    child?: Snippet<[{ props: Record<string, unknown> }]>;
    size?: "sm" | "md";
    isActive?: boolean;
  } = $props();

  const mergedProps = $derived({
    class: fcd(
      {
        "active": [
          "active:bg-sidebar-accent",
          "active:text-sidebar-accent-foreground",
          "data-[active=true]:text-sidebar-accent-foreground",
          "data-[active=true]:bg-sidebar-accent",
          "focus-visible:ring-2",
        ],
        "svg": [
          "[&>svg]:shrink-0",
          "[&>svg]:size-4",
          "[&>svg]:text-sidebar-accent-foreground",
        ],
        "hover": [
          "hover:bg-sidebar-accent",
          "hover:text-sidebar-accent-foreground",
        ],
        "disabled": [
          "disabled:pointer-events-none",
          "disabled:opacity-50",
          "aria-disabled:pointer-events-none",
          "aria-disabled:opacity-50",
        ],
        "border": [
          "rounded-md",
          "ring-sidebar-ring",
          "outline-hidden",
        ],
        "collapsible": "group-data-[collapsible=icon]:hidden",
        "layout": [
          "flex",
          "h-7",
          "min-w-0",
          "-translate-x-px",
          "items-center",
          "gap-2",
          "px-2",
          "overflow-hidden",
        ],
        "text": {
          "text-sidebar-foreground": true,
          "text-xs": size === "sm",
          "text-sm": size === "md",
        },
      },
      "[&>span:last-child]:truncate",
      className,
    ),

    "data-slot": "sidebar-menu-sub-button",
    "data-sidebar": "menu-sub-button",
    "data-size": size,
    "data-active": isActive,
    ...restProps,
  });
</script>

{#if child}
  {@render child({ props: mergedProps })}
{:else}
  <a bind:this={ref} {...mergedProps}>
    {@render children?.()}
  </a>
{/if}
