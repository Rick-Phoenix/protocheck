<script lang="ts">
  import HouseIcon from "@lucide/svelte/icons/house";
  import SettingsIcon from "@lucide/svelte/icons/settings";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import Button from "./ui/button/button.svelte";
  import SidebarFooterMenu from "./sidebar-footer-menu.svelte";

  const items = [
    {
      title: "Home",
      url: "/",
      icon: HouseIcon,
    },
    {
      title: "Settings",
      url: "/settings",
      icon: SettingsIcon,
    },
  ];
</script>

<Sidebar.Root
  variant="inset"
  collapsible="icon"
>
  <Sidebar.Header>
    Header
  </Sidebar.Header>
  <Sidebar.Content>
    <Sidebar.Group>
      <Sidebar.GroupLabel>Group</Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each items as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton>
                {#snippet child({ props })}
                  <a href={item.url} {...props}>
                    <item.icon />
                    <span>{item.title}</span>
                  </a>
                {/snippet}
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
  </Sidebar.Content>
  <Sidebar.Footer class="flex justify-between">
    <SidebarFooterMenu />
    <Button
      variant="ghost"
      size="icon"
      class="rounded-2xl"
      href={"/settings"}
    >
      <SettingsIcon />
    </Button>
  </Sidebar.Footer>
</Sidebar.Root>
