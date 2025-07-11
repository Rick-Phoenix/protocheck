<script lang="ts">
  import { Dialog as SheetPrimitive } from "bits-ui";
  import { fcd } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    ...restProps
  }: SheetPrimitive.OverlayProps = $props();
</script>

<SheetPrimitive.Overlay
  bind:ref
  data-slot="sheet-overlay"
  class={fcd({
    "animation": [
      "data-[state=open]:animate-in",
      "data-[state=closed]:animate-out",
      "data-[state=closed]:fade-out-0",
      "data-[state=open]:fade-in-0",
    ],
    "positioning": [
      "fixed",
      "inset-0",
      "z-50",
    ],
    "bg": "bg-black/50",
  }, className)}
  {...restProps}
/>
