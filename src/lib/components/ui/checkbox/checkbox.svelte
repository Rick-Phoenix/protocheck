<script lang="ts">
  import { cn } from "$lib/utils";
  import type { HTMLInputAttributes } from "svelte/elements";

  let {
    class: className,
    defaultChecked,
    checked = $bindable(defaultChecked),
    indeterminate = $bindable(false),
    ...restProps
  }: HTMLInputAttributes = $props();
</script>

<input
  type="checkbox"
  class={cn("checkbox not-checked:bg-muted checked:bg-accent", className)}
  bind:checked
  bind:indeterminate
  {...restProps}
/>
