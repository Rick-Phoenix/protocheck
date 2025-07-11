<script lang="ts">
  import { ModeWatcher } from "mode-watcher";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/app-sidebar.svelte";
  import "../app.css";
  import type { Side } from "$lib/components/ui/sidebar/context.svelte";
  import Separator from "$lib/components/ui/separator/separator.svelte";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js";
  import { Toaster } from "svelte-french-toast";
  import { page } from "$app/state";
  import { title } from "radashi";

  let { children } = $props();
  let sidebarSide = $state<Side>("left");
  const pageObj = page;

  const [parentRoute, childRoute] = $derived.by(() => {
    const fullRoute = pageObj.route.id;
    if (fullRoute) {
      const routeBits = fullRoute.split("/");
      return [routeBits[1], routeBits[2]];
    } else {
      return [];
    }
  });

  const joinUrlSegments = (...segments: string[]) =>
    "/" + segments.join("/");
</script>

<ModeWatcher defaultTheme="dark" />
<Toaster
  position="top-left"
  toastOptions={{ style: "background: var(--background); color: var(--primary)" }}
/>
<Sidebar.Provider bind:side={sidebarSide}>
  <AppSidebar />
  <Sidebar.Inset>
    <header class="flex h-12 shrink-0 items-center gap-2 border-b px-4">
      <Sidebar.Trigger class="-ml-1" />
      <Separator orientation="vertical" class="mr-2 h-4" />
      <Breadcrumb.Root>
        <Breadcrumb.List>
          {#if parentRoute}
            <Breadcrumb.Item class="hidden md:block">
              <Breadcrumb.Link href={"/" + parentRoute} active={!childRoute}>{
                title(parentRoute)
              }</Breadcrumb.Link>
            </Breadcrumb.Item>
          {/if}
          {#if parentRoute && childRoute}
            <Breadcrumb.Separator class="hidden md:block" />
            <Breadcrumb.Item>
              <Breadcrumb.Link
                href={joinUrlSegments(parentRoute, childRoute)}
                active={!!childRoute}
              >{title(childRoute)}</Breadcrumb.Link>
            </Breadcrumb.Item>
          {/if}
        </Breadcrumb.List>
      </Breadcrumb.Root>
    </header>
    {@render children?.()}
  </Sidebar.Inset>
</Sidebar.Provider>
