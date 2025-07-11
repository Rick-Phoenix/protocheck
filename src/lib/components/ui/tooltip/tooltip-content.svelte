<script lang="ts">
  import { Tooltip as TooltipPrimitive } from "bits-ui";
  import { fcd } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    sideOffset = 0,
    side = "top",
    children,
    arrowClasses,
    ...restProps
  }: TooltipPrimitive.ContentProps & {
    arrowClasses?: string;
  } = $props();
</script>

<TooltipPrimitive.Portal>
  <TooltipPrimitive.Content
    bind:ref
    data-slot="tooltip-content"
    {sideOffset}
    {side}
    class={fcd({
      "bg": ["bg-primary"],
      "text": [
        "text-primary-foreground",
        "text-balance",
        "text-xs",
      ],
      "border": "rounded-md",
      "animation": [
        "animate-in",
        "fade-in-0",
        "zoom-in-95",
        "data-[state=closed]:animate-out",
        "data-[state=closed]:fade-out-0",
        "data-[state=closed]:zoom-out-95",
        "data-[side=bottom]:slide-in-from-top-2",
        "data-[side=left]:slide-in-from-right-2",
        "data-[side=right]:slide-in-from-left-2",
        "data-[side=top]:slide-in-from-bottom-2",
      ],
      "transform": [
        "origin-(--bits-tooltip-content-transform-origin)",
        "z-50",
      ],
      "layout": [
        "w-fit",
        "px-3",
        "py-1.5",
      ],
    }, className)}
    {...restProps}
  >
    {@render children?.()}
    <TooltipPrimitive.Arrow>
      {#snippet child({ props })}
        <div
          class={fcd({
            "bg": ["bg-primary"],
            "border": "rounded-[2px]",
            "side=top": [
              "data-[side=top]:translate-x-1/2",
              "data-[side=top]:translate-y-[calc(-50%_+_2px)]",
            ],
            "side=bottom": [
              "data-[side=bottom]:-translate-x-1/2",
              "data-[side=bottom]:-translate-y-[calc(-50%_+_1px)]",
            ],
            "side=left": [
              "data-[side=left]:-translate-y-[calc(50%_-_3px)]",
            ],
            "side=right": [
              "data-[side=right]:translate-x-[calc(50%_+_2px)]",
              "data-[side=right]:translate-y-1/2",
            ],
            "transform": ["z-50", "rotate-45"],
            "layout": "size-2.5",
          }, arrowClasses)}
          {...props}
        >
        </div>
      {/snippet}
    </TooltipPrimitive.Arrow>
  </TooltipPrimitive.Content>
</TooltipPrimitive.Portal>
