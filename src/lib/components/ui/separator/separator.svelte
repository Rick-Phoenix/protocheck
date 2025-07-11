<script lang="ts">
  import { Separator as SeparatorPrimitive } from "bits-ui";
  import { fcd } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    ...restProps
  }: SeparatorPrimitive.RootProps = $props();
</script>

<SeparatorPrimitive.Root
  bind:ref
  data-slot="separator"
  class={fcd({
    "bg": [
      "bg-border",
    ],
    "layout": [
      "shrink-0",
      "data-[orientation=horizontal]:h-px",
      "data-[orientation=vertical]:h-full",
      "data-[orientation=horizontal]:w-full",
      "data-[orientation=vertical]:w-px",
    ],
  }, className)}
  {...restProps}
/>
