<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<div class="flex-col-center size-full gap-4">
  <h1 class="text-center">Welcome to Tauri + Svelte</h1>
  <div class="flex-center gap-3">
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="size-24" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank">
      <img src="/svelte.svg" class="size-24" alt="SvelteKit Logo" />
    </a>
  </div>
  <p>Click on the Tauri and SvelteKit logos to learn more.</p>

  <form class="flex-center" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</div>
