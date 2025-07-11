<script lang="ts">
  import { ModeWatcher } from "mode-watcher";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/app-sidebar.svelte";
  import "../app.css";
  import type { Side } from "$lib/components/ui/sidebar/context.svelte";
  import Separator from "$lib/components/ui/separator/separator.svelte";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js";
  import { Toaster } from "svelte-french-toast";

  let { children } = $props();
  let sidebarSide = $state<Side>("left");
</script>

<ModeWatcher defaultTheme="dark" />
<Toaster
  position="top-left"
  toastOptions={{ style: "background: var(--background); color: var(--primary)" }}
/>
<Sidebar.Provider bind:side={sidebarSide}>
  <AppSidebar />
  <Sidebar.Inset>
    <header class="flex h-16 shrink-0 items-center gap-2 border-b px-4">
      <Sidebar.Trigger class="-ml-1" />
      <Separator orientation="vertical" class="mr-2 h-4" />
      <Breadcrumb.Root>
        <Breadcrumb.List>
          <Breadcrumb.Item class="hidden md:block">
            <Breadcrumb.Link href="#"
            >Building Your Application</Breadcrumb.Link>
          </Breadcrumb.Item>
          <Breadcrumb.Separator class="hidden md:block" />
          <Breadcrumb.Item>
            <Breadcrumb.Page>Data Fetching</Breadcrumb.Page>
          </Breadcrumb.Item>
        </Breadcrumb.List>
      </Breadcrumb.Root>
    </header>
    <main class="p-5 size-full flex-center">
      {@render children?.()}
    </main>
  </Sidebar.Inset>
</Sidebar.Provider>
