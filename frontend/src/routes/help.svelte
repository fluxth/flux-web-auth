<script context="module" lang="ts">
  import type { Load } from "@sveltejs/kit";

  export const load: Load = async ({ url }) => {
    return { props: { backUrl: url.searchParams.get("back"), currentUrl: url } };
  };
</script>

<script lang="ts">
  import Card from "../components/Card.svelte";
  import HeaderIcon from "eva-icons/fill/svg/question-mark.svg";
  import BackIcon from "eva-icons/outline/svg/arrow-back-outline.svg";
  import { page } from "$app/stores";

  export let currentUrl;
  export let backUrl;

  $: {
    try {
      const url = new URL(backUrl);
      if (url.host !== currentUrl.host || url.protocol !== currentUrl.protocol) backUrl = null;
    } catch {
      backUrl = null;
    }

    backUrl = backUrl ? backUrl : "/";
  }
</script>

<Card showHelp={false}>
  <a href={backUrl} class="absolute -ml-3 -mt-2">
    <BackIcon width="30" class="fill-stone-400 hover:fill-stone-500 transition" />
  </a>
  <div class="text-center mb-2">
    <HeaderIcon width="50" class="mx-auto mb-2 fill-teal-700" />
    <h1 class="text-xl font-medium">What's this?</h1>
    <p class="text-stone-500 dark:text-stone-400 font-light break-words" />
  </div>
  <div class="text-center mb-4 text-sm">
    <span class="px-2 text-xs bg-teal-700 text-white rounded">TL;DR</span>
    <p class="mt-1 mb-3">An authentication service for flux.ci</p>
    <span class="px-2 text-xs bg-teal-700 text-white rounded">What again?</span>
    <p class="mt-1">
      Basically a login page for flux.ci web services, read more
      <a href="#" on:click|preventDefault class="underline text-teal-700">here</a>.
    </p>
  </div>

  <div class="text-center text-xs text-stone-500">
    <p class="mb-1">
      Source code avalible on
      <a class="underline" rel="external" href="https://github.com/fluxth/flux-web-auth">GitHub</a>
    </p>
    <p>Version: 1.0.0</p>
    <p>Build: DEVELOPMENT</p>
  </div>
</Card>
