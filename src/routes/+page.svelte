<script lang="ts">
  import { GreetService } from "../../bindings/changeme";
  import { Events } from "@wailsio/runtime";

  let name = $state("");
  let result = $state("Please enter your name below 👇");
  let time = $state("Listening for Time event...");

  const doGreet = (): void => {
    let localName = name;
    if (!localName) {
      localName = "anonymous";
    }
    GreetService.Greet(localName).then((resultValue: string) => {
      result = resultValue;
    }).catch((err: any) => {
      console.log(err);
    });
  };

  Events.On("time", (timeValue: any) => {
    time = timeValue.data;
  });
</script>

<div class="flex-col-center size-full">
  <div class="flex h-min">
    <span data-wml-openURL="https://wails.io">
      <img src="/wails.png" class="logo" alt="Wails logo" />
    </span>
    <span data-wml-openURL="https://svelte.dev">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte logo" />
    </span>
  </div>
  <h1>Wails + Svelte</h1>
  <div class="flex-col-center gap-3">
    <div>{result}</div>
    <div class="card">
      <div class="flex gap-3">
        <input class="input" bind:value={name} type="text" autocomplete="off" />
        <button class="btn" onclick={doGreet}>Greet</button>
      </div>
    </div>
  </div>
</div>
