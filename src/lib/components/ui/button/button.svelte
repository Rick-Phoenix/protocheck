<script lang="ts" module>
  import { cn, fcd, type WithElementRef } from "$lib/utils.js";
  import type {
    HTMLAnchorAttributes,
    HTMLButtonAttributes,
  } from "svelte/elements";
  import { tv, type VariantProps } from "tailwind-variants";

  export const buttonVariants = tv({
    base: fcd({
      "svg": [
        "[&_svg:not([class*='size-'])]:size-4",
        "[&_svg]:shrink-0",
        "[&_svg]:pointer-events-none",
      ],
      "disabled": [
        "aria-disabled:pointer-events-none",
        "aria-disabled:opacity-50",
        "disabled:opacity-50",
        "disabled:pointer-events-none",
      ],
      "text": [
        "whitespace-nowrap",
        "font-medium",
        "text-sm",
      ],
      "transition": "transition-all",
      "invalid": [
        "aria-invalid:ring-destructive/20",
        "aria-invalid:border-destructive",
        "dark:aria-invalid:ring-destructive/40",
      ],
      "border": [
        "rounded-md",
        "outline-none",
      ],
      "layout": [
        "inline-flex",
        "shrink-0",
        "items-center",
        "justify-center",
        "gap-2",
        "focus-visible:border-ring",
        "focus-visible:ring-ring/50",
        "focus-visible:ring-[3px]",
      ],
    }),
    variants: {
      variant: {
        default: fcd({
          "bg": ["bg-primary", "hover:bg-primary/90"],
          "text": "text-shadow-primary-foreground",
          "border": "shadow-xs",
        }),
        destructive: fcd({
          "bg": [
            "bg-destructive",
            "dark:bg-destructive/60",
            "hover:bg-destructive/90",
          ],
          "border": [
            "shadow-xs",
            "focus-visible:ring-destructive/20",
            "dark:focus-visible:ring-destructive/40",
          ],
          "text": "text-white",
        }),
        outline: fcd({
          "bg": [
            "bg-background",
            "dark:bg-input/30",
            "dark:hover:bg-input/50",
            "hover:bg-accent",
            "hover:text-accent-foreground",
          ],
          "border": [
            "shadow-xs",
            "dark:border-input",
            "border",
          ],
        }),
        secondary: fcd({
          "bg": [
            "bg-secondary",
            "hover:bg-secondary/80",
          ],
          "text": "text-shadow-secondary-foreground",
          "border": "shadow-xs",
        }),
        ghost: fcd({
          "bg": [
            "hover:bg-accent",
            "dark:hover:bg-accent/50",
          ],
          "text": "hover:text-accent-foreground",
        }),
        link: fcd({
          "text": [
            "text-primary",
            "underline-offset-4",
            "hover:underline",
          ],
        }),
      },
      size: {
        default: "h-9 px-4 py-2 has-[>svg]:px-3",
        sm: "h-8 gap-1.5 rounded-md px-3 has-[>svg]:px-2.5",
        lg: "h-10 rounded-md px-6 has-[>svg]:px-4",
        icon: "size-9",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  });

  export type ButtonVariant = VariantProps<
    typeof buttonVariants
  >["variant"];
  export type ButtonSize = VariantProps<typeof buttonVariants>["size"];

  export type ButtonProps =
    & WithElementRef<HTMLButtonAttributes>
    & WithElementRef<HTMLAnchorAttributes>
    & {
      variant?: ButtonVariant;
      size?: ButtonSize;
    };
</script>

<script lang="ts">
  let {
    class: className,
    variant = "default",
    size = "default",
    ref = $bindable(null),
    href = undefined,
    type = "button",
    disabled,
    children,
    ...restProps
  }: ButtonProps = $props();
</script>

{#if href}
  <a
    bind:this={ref}
    data-slot="button"
    class={cn(buttonVariants({ variant, size }), className)}
    href={disabled ? undefined : href}
    aria-disabled={disabled}
    role={disabled ? "link" : undefined}
    tabindex={disabled ? -1 : undefined}
    {...restProps}
  >
    {@render children?.()}
  </a>
{:else}
  <button
    bind:this={ref}
    data-slot="button"
    class={cn(buttonVariants({ variant, size }), className)}
    {type}
    {disabled}
    {...restProps}
  >
    {@render children?.()}
  </button>
{/if}
