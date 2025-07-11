<script lang="ts">
  import type { HTMLAnchorAttributes } from "svelte/elements";
  import type { Snippet } from "svelte";
  import { cn, type WithElementRef } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    href = undefined,
    child,
    children,
    active = false,
    ...restProps
  }: WithElementRef<HTMLAnchorAttributes> & {
    child?: Snippet<[{ props: HTMLAnchorAttributes }]>;
    active?: boolean;
  } = $props();

  const attrs = $derived({
    "data-active": active,
    "data-slot": "breadcrumb-link",
    class: cn(
      "hover:text-foreground data-[active=true]:text-foreground transition-colors",
      className,
    ),
    href,
    ...restProps,
  });
</script>

{#if child}
  {@render child({ props: attrs })}
{:else}
  <a bind:this={ref} {...attrs}>
    {@render children?.()}
  </a>
{/if}
