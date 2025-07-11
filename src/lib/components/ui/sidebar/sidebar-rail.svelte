<script lang="ts">
  import { fcd, type WithElementRef } from "$lib/utils.js";
  import type { HTMLAttributes } from "svelte/elements";
  import { useSidebar } from "./context.svelte.js";

  let {
    ref = $bindable(null),
    class: className,
    children,
    ...restProps
  }: WithElementRef<HTMLAttributes<HTMLButtonElement>, HTMLButtonElement> =
    $props();

  const sidebar = useSidebar();
</script>

<button
  bind:this={ref}
  data-sidebar="rail"
  data-slot="sidebar-rail"
  aria-label="Toggle Sidebar"
  tabIndex={-1}
  onclick={sidebar.toggle}
  title="Toggle Sidebar"
  class={fcd({
    "hover": [
      "hover:after:bg-sidebar-border",
      "hover:group-data-[collapsible=offcanvas]:bg-sidebar",
    ],
    "after": [
      "after:absolute",
      "after:w-[2px]",
      "after:inset-y-0",
      "after:left-[calc(1/2*100%-1px)]",
    ],
    "side=left": [
      "group-data-[side=left]:-right-4",
      "in-data-[side=left]:cursor-w-resize",
      "[[data-side=left][data-state=collapsed]_&]:cursor-e-resize",
    ],
    "side=right": [
      "group-data-[side=right]:left-0",
      "in-data-[side=right]:cursor-e-resize",
      "[[data-side=right][data-state=collapsed]_&]:cursor-w-resize",
    ],
    "collapsible": [
      "group-data-[collapsible=offcanvas]:translate-x-0",
      "group-data-[collapsible=offcanvas]:after:left-full",
      "[[data-side=left][data-collapsible=offcanvas]_&]:-right-2",
      "[[data-side=right][data-collapsible=offcanvas]_&]:-left-2",
    ],
    "transition": "transition-all ease-linear",
    "layout": "absolute inset-y-0 z-20 hidden w-4 -translate-x-1/2 sm:flex",
  }, className)}
  {...restProps}
>
  {@render children?.()}
</button>
