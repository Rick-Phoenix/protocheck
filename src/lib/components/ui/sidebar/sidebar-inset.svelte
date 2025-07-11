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

<main
  bind:this={ref}
  data-slot="sidebar-inset"
  class={fcd({
    "layout": [
      "relative",
      "flex",
      "w-full",
      "flex-1",
      "flex-col",
    ],
    "bg": "bg-background",
    "md": [
      "md:peer-data-[variant=inset]:m-2",
      "md:peer-data-[variant=inset]:peer-data-[side=left]:ml-0",
      "md:peer-data-[variant=inset]:peer-data-[side=right]:mr-0",
      "md:peer-data-[variant=inset]:peer-data-[side=left]:peer-data-[state=collapsed]:mr-2",
      "md:peer-data-[variant=inset]:peer-data-[side=right]:peer-data-[state=collapsed]:ml-2",
      "md:peer-data-[variant=inset]:rounded-xl",
      "md:peer-data-[variant=inset]:shadow-sm",
    ],
  }, className)}
  {...restProps}
>
  {@render children?.()}
</main>
