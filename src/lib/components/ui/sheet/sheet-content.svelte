<script lang="ts" module>
  import { tv, type VariantProps } from "tailwind-variants";
  export const sheetVariants = tv({
    base: fcd({
      "bg": "bg-background",
      "animation": [
        "data-[state=open]:animate-in",
        "data-[state=closed]:animate-out",
      ],
      "layout": [
        "flex",
        "flex-col",
        "gap-4",
      ],
      "transition": [
        "transition",
        "ease-in-out",
        "data-[state=closed]:duration-300",
        "data-[state=open]:duration-500",
      ],
      "border": "shadow-lg",
      "positioning": [
        "fixed",
        "z-50",
      ],
    }),
    variants: {
      side: {
        top: fcd({
          "border": ["border-b", "inset-x-0"],
          "animation": [
            "data-[state=closed]:slide-out-to-top",
            "data-[state=open]:slide-in-from-top",
          ],
          "layout": "h-auto",
          "positioning": "top-0",
        }),

        bottom: fcd({
          "animation": [
            "data-[state=closed]:slide-out-to-bottom",
            "data-[state=open]:slide-in-from-bottom",
          ],
          "border": [
            "inset-x-0",
            "border-t",
          ],
          "layout": "h-auto",
          "positioning": "bottom-0",
        }),

        left: fcd({
          "animation": [
            "data-[state=closed]:slide-out-to-left",
            "data-[state=open]:slide-in-from-left",
          ],
          "layout": [
            "h-full",
            "sm:max-w-sm",
            "w-3/4",
          ],
          "border": [
            "border-r",
            "inset-y-0",
          ],
          "positioning": "left-0",
        }),

        right: fcd({
          "animation": [
            "data-[state=closed]:slide-out-to-right",
            "data-[state=open]:slide-in-from-right",
          ],
          "border": [
            "inset-y-0",
            "border-l",
          ],
          "layout": [
            "h-full",
            "w-3/4",
            "sm:max-w-sm",
          ],
          "positioning": "right-0",
        }),
      },
    },
    defaultVariants: {
      side: "right",
    },
  });

  export type Side = VariantProps<typeof sheetVariants>["side"];
</script>

<script lang="ts">
  import { Dialog as SheetPrimitive } from "bits-ui";
  import XIcon from "@lucide/svelte/icons/x";
  import type { Snippet } from "svelte";
  import SheetOverlay from "./sheet-overlay.svelte";
  import { cn, fcd, type WithoutChildrenOrChild } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    side = "right",
    portalProps,
    children,
    ...restProps
  }: WithoutChildrenOrChild<SheetPrimitive.ContentProps> & {
    portalProps?: SheetPrimitive.PortalProps;
    side?: Side;
    children: Snippet;
  } = $props();
</script>

<SheetPrimitive.Portal {...portalProps}>
  <SheetOverlay />
  <SheetPrimitive.Content
    bind:ref
    data-slot="sheet-content"
    class={cn(sheetVariants({ side }), className)}
    {...restProps}
  >
    {@render children?.()}
    <SheetPrimitive.Close
      class={fcd({
        "border": [
          "rounded-xs",
          "ring-offset-background",
          "focus-visible:ring-ring",
          "focus-visible:outline-hidden",
          "focus-visible:ring-2",
          "focus-visible:ring-offset-2",
        ],
        "disabled": "disabled:pointer-events-none",
        "positioning": [
          "absolute",
          "right-4",
          "top-4",
        ],
        "opacity": [
          "opacity-70",
          "transition-opacity",
          "hover:opacity-100",
        ],
      })}
    >
      <XIcon class="size-4" />
      <span class="sr-only">Close</span>
    </SheetPrimitive.Close>
  </SheetPrimitive.Content>
</SheetPrimitive.Portal>
