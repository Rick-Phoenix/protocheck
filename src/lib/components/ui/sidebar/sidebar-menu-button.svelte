<script lang="ts" module>
  import { tv, type VariantProps } from "tailwind-variants";
  import { fcd } from "$lib/utils.js";

  export const sidebarMenuButtonVariants = tv({
    base: fcd(
      {
        "hover": [
          "hover:bg-sidebar-accent",
          "hover:text-sidebar-accent-foreground",
        ],
        "active": [
          "active:bg-sidebar-accent",
          "active:text-sidebar-accent-foreground",
          "data-[state=open]:hover:text-sidebar-accent-foreground",
          "data-[state=open]:hover:bg-sidebar-accent",
          "data-[active=true]:bg-sidebar-accent",
        ],
        "border": [
          "rounded-md",
          "outline-hidden",
          "ring-sidebar-ring",
          "focus-visible:ring-2",
        ],
        "disabled": [
          "disabled:pointer-events-none",
          "disabled:opacity-50",
          "aria-disabled:pointer-events-none",
          "aria-disabled:opacity-50",
        ],
        "svg": [
          "[&>svg]:shrink-0",
          "[&>svg]:size-4",
        ],
        "collapsible": [
          "group-data-[collapsible=icon]:size-8!",
          "group-data-[collapsible=icon]:p-2!",
        ],

        "text": [
          "text-left",
          "text-sm",
          "data-[active=true]:font-medium",
          "data-[active=true]:text-sidebar-accent-foreground",
        ],
        "peer": "peer/menu-button",
        "layout": [
          "flex",
          "w-full",
          "items-center",
          "gap-2",
          "p-2",
          "overflow-hidden",
        ],
        "transition": "transition-[width,height,padding]",
        "last-child": "[&>span:last-child]:truncate",
        "sidebar-menu-action":
          "group-has-data-[sidebar=menu-action]/menu-item:pr-8",
      },
    ),
    variants: {
      variant: {
        default:
          "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
        outline: fcd({
          "bg": "bg-background",
          "hover": [
            "hover:bg-sidebar-accent",
            "hover:text-sidebar-accent-foreground",
          ],
          "shadow": [
            "shadow-[0_0_0_1px_var(--sidebar-border)]",
            "hover:shadow-[0_0_0_1px_var(--sidebar-accent)]",
          ],
        }),
      },
      size: {
        default: "h-8 text-sm",
        sm: "h-7 text-xs",
        lg: "group-data-[collapsible=icon]:p-0! h-12 text-sm",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  });

  export type SidebarMenuButtonVariant = VariantProps<
    typeof sidebarMenuButtonVariants
  >["variant"];
  export type SidebarMenuButtonSize = VariantProps<
    typeof sidebarMenuButtonVariants
  >["size"];
</script>

<script lang="ts">
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import {
    cn,
    type WithElementRef,
    type WithoutChildrenOrChild,
  } from "$lib/utils.js";
  import { mergeProps } from "bits-ui";
  import type { ComponentProps, Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";
  import { useSidebar } from "./context.svelte.js";

  let {
    ref = $bindable(null),
    class: className,
    children,
    child,
    variant = "default",
    size = "default",
    isActive = false,
    tooltipContent,
    tooltipContentProps,
    ...restProps
  }:
    & WithElementRef<HTMLAttributes<HTMLButtonElement>, HTMLButtonElement>
    & {
      isActive?: boolean;
      variant?: SidebarMenuButtonVariant;
      size?: SidebarMenuButtonSize;
      tooltipContent?: Snippet | string;
      tooltipContentProps?: WithoutChildrenOrChild<
        ComponentProps<typeof Tooltip.Content>
      >;
      child?: Snippet<[{ props: Record<string, unknown> }]>;
    } = $props();

  const sidebar = useSidebar();

  const buttonProps = $derived({
    class: cn(sidebarMenuButtonVariants({ variant, size }), className),
    "data-slot": "sidebar-menu-button",
    "data-sidebar": "menu-button",
    "data-size": size,
    "data-active": isActive,
    ...restProps,
  });
</script>

{#snippet Button({ props }: { props?: Record<string, unknown> })}
  {@const mergedProps = mergeProps(buttonProps, props)}
  {#if child}
    {@render child({ props: mergedProps })}
  {:else}
    <button bind:this={ref} {...mergedProps}>
      {@render children?.()}
    </button>
  {/if}
{/snippet}

{#if !tooltipContent}
  {@render Button({})}
{:else}
  <Tooltip.Root>
    <Tooltip.Trigger>
      {#snippet child({ props })}
        {@render Button({ props })}
      {/snippet}
    </Tooltip.Trigger>
    <Tooltip.Content
      side="right"
      align="center"
      hidden={sidebar.state !== "collapsed" || sidebar.isMobile}
      {...tooltipContentProps}
    >
      {#if typeof tooltipContent === "string"}
        {tooltipContent}
      {:else if tooltipContent}
        {@render tooltipContent()}
      {/if}
    </Tooltip.Content>
  </Tooltip.Root>
{/if}
