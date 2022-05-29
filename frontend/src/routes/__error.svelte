<script context="module" lang="ts">
  export function load({ error, status }) {
    return {
      props: {
        error,
        status,
      },
    };
  }
</script>

<script lang="ts">
  import Card from "../components/Card.svelte";
  import HeaderIcon from "eva-icons/outline/svg/alert-triangle-outline.svg";

  export let error;
  export let status;
</script>

<Card>
  <div class="text-center mb-6">
    <HeaderIcon width="50" class="mx-auto fill-red-500" />
    <div class="-mt-1 mb-2 font-bold text-red-500">{status}</div>
    <h1 class="text-xl font-medium">An error occurred!</h1>
    <p class="text-stone-500 dark:text-stone-400 font-light break-words">{error.message}</p>
  </div>
  <div class="text-sm text-center">
    <a href="/" class="block px-6 py-2 text-stone-600 bg-stone-50 border shadow hover:shadow-lg
    hover:bg-stone-100 active:bg-stone-200 rounded-md transition w-full">
    Back to homepage
    </a>
  </div>
</Card>
