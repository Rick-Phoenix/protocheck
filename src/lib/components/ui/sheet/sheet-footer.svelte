<script lang="ts">
  import { fcd, type WithElementRef } from "$lib/utils.js";
  import type { HTMLAttributes } from "svelte/elements";

  let {
    ref = $bindable(null),
    class: className,
    children,
    ...restProps
  }: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
  bind:this={ref}
  data-slot="sheet-footer"
  class={fcd({
    "layout": "flex flex-col",
    "spacing": ["mt-auto", "gap-2", "p-4"],
  }, className)}
  {...restProps}
>
  {@render children?.()}
</div>
