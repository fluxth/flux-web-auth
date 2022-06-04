<script lang="ts">
  import AlertIcon from "eva-icons/outline/svg/alert-circle-outline.svg";
  import { page } from "$app/stores";

  export let showHelp: boolean = true;
  $: pageUrl = encodeURIComponent($page.url.href);
</script>

<div class="sm:w-96 mx-auto">
  <div class="flex flex-col h-screen justify-center px-4 gap-2">
    <noscript>
      <div class="flex items-center gap-2 py-3 px-4 bg-red-200 rounded text-red-800 text-sm">
        <AlertIcon width="20" class="inline fill-red-800" />
        Please enable JavaScript to run this app!
      </div>
    </noscript>
    <div
      class="relative overflow-clip p-8 border border-stone-200 dark:border-stone-700 shadow-lg rounded-md dark:bg-stone-900"
    >
      <slot />
    </div>
    <div class="px-1 flex justify-between items-center text-stone-400 font-light text-[0.6rem]">
      <span>&copy; 2022, flux.ci</span>
      <div class="flex gap-2 underline">
        {#if showHelp}
          <a href="/help?back={pageUrl}">What's this?</a>
        {/if}
      </div>
    </div>
  </div>
</div>
