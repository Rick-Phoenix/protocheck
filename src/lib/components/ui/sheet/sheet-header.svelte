<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";
  import { fcd, type WithElementRef } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    children,
    ...restProps
  }: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
  bind:this={ref}
  data-slot="sheet-header"
  class={fcd({
    "layout": "flex flex-col",
    "spacing": "gap-1.5 p-4",
  }, className)}
  {...restProps}
>
  {@render children?.()}
</div>
