<script lang="ts">
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import { fcd, type WithElementRef } from "$lib/utils.js";
  import type { HTMLAttributes } from "svelte/elements";
  import {
    SIDEBAR_COOKIE_MAX_AGE,
    SIDEBAR_COOKIE_NAME,
    SIDEBAR_WIDTH,
    SIDEBAR_WIDTH_ICON,
  } from "./constants.js";
  import { setSidebar, type Side } from "./context.svelte.js";


  let {
    ref = $bindable(null),
    open = $bindable(true),
    openOnHover = $bindable(false),
    side = $bindable<Side>("left"),
    onOpenChange = () => {},
    class: className,
    style,
    children,
    ...restProps
  }: WithElementRef<HTMLAttributes<HTMLDivElement>> & {
    open?: boolean;
    onOpenChange?: (open: boolean) => void;
    openOnHover?: boolean;
    side?: Side;
  } = $props();

  const sidebar = setSidebar({
    side: () => side,
    openOnHover: () => openOnHover,
    open: () => open,
    setOpen: (value: boolean) => {
      open = value;
      onOpenChange(value);

      // This sets the cookie to keep the sidebar state.
      document.cookie =
        `${SIDEBAR_COOKIE_NAME}=${open}; path=/; max-age=${SIDEBAR_COOKIE_MAX_AGE}`;
    },
  });
</script>

<svelte:window onkeydown={sidebar.handleShortcutKeydown} />

<Tooltip.Provider delayDuration={0}>
  <div
    data-slot="sidebar-wrapper"
    style="--sidebar-width: {SIDEBAR_WIDTH}; --sidebar-width-icon: {SIDEBAR_WIDTH_ICON}; {style}"
    class={fcd(
      {
        "layout": "flex min-h-svh w-full",
        "group": "group/sidebar-wrapper",
        "bg": "has-data-[variant=inset]:bg-sidebar",
      },
      className,
    )}
    bind:this={ref}
    {...restProps}
  >
    {@render children?.()}
  </div>
</Tooltip.Provider>
