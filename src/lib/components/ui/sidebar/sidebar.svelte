<script lang="ts">
  import * as Sheet from "$lib/components/ui/sheet/index.js";
  import { cn, fcd, type WithElementRef } from "$lib/utils.js";
  import type { HTMLAttributes } from "svelte/elements";
  import { SIDEBAR_WIDTH_MOBILE } from "./constants.js";
  import { useSidebar } from "./context.svelte.js";

  let {
    ref = $bindable(null),
    variant = "sidebar",
    collapsible = "offcanvas",
    class: className,
    children,
    ...restProps
  }: WithElementRef<HTMLAttributes<HTMLDivElement>> & {
    variant?: "sidebar" | "floating" | "inset";
    collapsible?: "offcanvas" | "icon" | "none";
  } = $props();

  const isFloatingOrInset = variant === "floating" || variant === "inset";
  const sidebarState = useSidebar();
</script>

{#if collapsible === "none"}
  <div
    class={cn(
      "bg-sidebar text-sidebar-foreground w-(--sidebar-width) flex h-full flex-col",
      className,
    )}
    bind:this={ref}
    {...restProps}
  >
    {@render children?.()}
  </div>
{:else if sidebarState.isMobile}
  <Sheet.Root
    bind:open={() => sidebarState.openMobile, (v) => sidebarState.setOpenMobile(v)}
    {...restProps}
  >
    <Sheet.Content
      data-sidebar="sidebar"
      data-slot="sidebar"
      data-mobile="true"
      class="bg-sidebar text-sidebar-foreground w-(--sidebar-width) p-0 [&>button]:hidden"
      style="--sidebar-width: {SIDEBAR_WIDTH_MOBILE}"
      side={sidebarState.side}
    >
      <Sheet.Header class="sr-only">
        <Sheet.Title>Sidebar</Sheet.Title>
        <Sheet.Description>Displays the mobile sidebar.</Sheet.Description>
      </Sheet.Header>
      <div class="flex h-full w-full flex-col">
        {@render children?.()}
      </div>
    </Sheet.Content>
  </Sheet.Root>
{:else}
  <div
    bind:this={ref}
    role="navigation"
    class={"data-[side=right]:order-last text-sidebar-foreground group peer hidden md:block"}
    data-state={sidebarState.state}
    data-collapsible={sidebarState.state === "collapsed" ? collapsible : ""}
    data-variant={variant}
    data-side={sidebarState.side}
    data-slot="sidebar"
    onmouseenter={() => {
      if (sidebarState.openOnHover) {
        sidebarState.setOpen(true);
      }
    }}
    onmouseleave={() => {
      if (sidebarState.openOnHover) {
        sidebarState.setOpen(false);
      }
    }}
  >
    <!-- This is what handles the sidebar gap on desktop -->
    <div
      data-slot="sidebar-gap"
      class={fcd({
        "transition": "transition-[width] duration-200 ease-linear",
        "collapsible": [
          "group-data-[collapsible=offcanvas]:w-0",
          isFloatingOrInset
            ? "group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4))+2px)]"
            : "group-data-[collapsible=icon]:w-(--sidebar-width-icon)",
        ],
        "layout": "w-(--sidebar-width) relative bg-transparent",
        "side": "group-data-[side=right]:rotate-180",
      })}
    >
    </div>
    <div
      data-slot="sidebar-container"
      class={fcd({
        "side": sidebarState.side === "left"
          ? "left-0 group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)]"
          : "right-0 group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)]",
        "transition":
          "transition-[left,right,width] duration-200 ease-linear",
        "collapsible": isFloatingOrInset
          ? "group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4))+2px)]"
          : "group-data-[collapsible=icon]:w-(--sidebar-width-icon)",
        "border": {
          "group-data-[side=left]:border-r group-data-[side=right]:border-l":
            !isFloatingOrInset,
        },
        "layout":
          "w-(--sidebar-width)  fixed inset-y-0 z-10 hidden h-full p-2  md:flex",
      })}
      {...restProps}
    >
      <div
        data-sidebar="sidebar"
        data-slot="sidebar-inner"
        class={fcd({
          "variant=floating": [
            "group-data-[variant=floating]:border-sidebar-border",
            "group-data-[variant=floating]:rounded-lg",
            "group-data-[variant=floating]:border",
            "group-data-[variant=floating]:shadow-sm",
          ],
          "layout+bg": "bg-sidebar flex h-full w-full flex-col",
        })}
      >
        {@render children?.()}
      </div>
    </div>
  </div>
{/if}
