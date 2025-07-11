<script lang="ts">
  import { fcd, type WithElementRef } from "$lib/utils.js";
  import type { HTMLAttributes } from "svelte/elements";

  let {
    ref = $bindable(null),
    class: className,
    children,
    ...restProps
  }: WithElementRef<HTMLAttributes<HTMLElement>> = $props();
</script>

<div
  bind:this={ref}
  data-slot="sidebar-menu-badge"
  data-sidebar="menu-badge"
  class={fcd({
    "text": [
      "text-sidebar-foreground",
      "text-xs",
      "font-medium",
      "tabular-nums",
    ],
    "interactivity": [
      "pointer-events-none",
      "select-none",
    ],
    "layout": [
      "absolute",
      "right-1",
      "flex",
      "h-5",
      "min-w-5",
      "items-center",
      "justify-center",
      "px-1",
    ],
    "border": "rounded-md",
    "peer/menu-button": [
      "peer-hover/menu-button:text-sidebar-accent-foreground",
      "peer-data-[active=true]/menu-button:text-sidebar-accent-foreground",
      "peer-data-[size=sm]/menu-button:top-1",
      "peer-data-[size=default]/menu-button:top-1.5",
      "peer-data-[size=lg]/menu-button:top-2.5",
    ],
    "collapsible": "group-data-[collapsible=icon]:hidden",
  })}
  {...restProps}
>
  {@render children?.()}
</div>
