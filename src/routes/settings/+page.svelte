<script lang="ts">
  import { useSidebar } from "$lib/components/ui/sidebar";

  import { toggleMode } from "mode-watcher";
  import { Button } from "$lib/components/ui/button/index.js";
  import Checkbox from "$lib/components/ui/checkbox/checkbox.svelte";

  const sidebar = useSidebar();
</script>

<h2 class="border-b p-4">Settings</h2>
<div class="p-4 flex-col gap-3">
  <label class="flex items-center gap-2">
    <Checkbox
      bind:checked={sidebar.openOnHover}
    />
    Open sidebar on hover
  </label>

  <div class="flex gap-3 items-center">
    <Button onclick={toggleMode} variant="outline" size="icon">
      <span
        class="icon-[lucide--sun] h-[1.2rem] w-[1.2rem] rotate-0 scale-100 !transition-all dark:-rotate-90 dark:scale-0"
      ></span>

      <span
        class="icon-[lucide--moon] absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 !transition-all dark:rotate-0 dark:scale-100"
      ></span>
      <span class="sr-only">Toggle theme</span>
    </Button>
    <span>Toggle theme</span>
  </div>
</div>
