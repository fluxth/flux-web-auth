<script context="module" lang="ts">
  import type { Load } from "@sveltejs/kit";

  type ErrorProp = {
    title?: string;
    message: string;
    code?: string;
  };

  export const load: Load = ({ error, status }) => {
    const errorProp: ErrorProp = {
      title: "An error occurred!",
      message: "An unknown error occurred",
    };

    if (error?.message) errorProp.message = error.message;

    if (error?.name === "RpcError") errorProp.title = "Connection Error";

    return {
      props: {
        error: errorProp,
        status,
      },
    };
  };
</script>

<script lang="ts">
  import Card from "../components/Card.svelte";
  import HeaderIcon from "eva-icons/outline/svg/alert-triangle-outline.svg";

  export let error: ErrorProp;
  export let status: number | null;
</script>

<Card>
  <div class="text-center mb-6">
    <HeaderIcon width="50" class="mx-auto fill-red-500" />
    <div class="-mt-1 mb-2 font-bold text-red-500">{status}</div>
    <h1 class="text-xl font-medium">{error.title}</h1>
    <p class="text-stone-500 dark:text-stone-400 font-light whitespace-pre-wrap break-words">
      {error.message}
    </p>
    {#if error.code}
      <p class="font-mono text-[0.6rem] text-stone-400 mt-2">
        [{error.code}]
      </p>
    {/if}
  </div>
  <div class="text-sm text-center">
    <a
      href="/"
      class="block px-6 py-2 text-stone-600 bg-stone-50 border shadow hover:shadow-lg
    hover:bg-stone-100 active:bg-stone-200 rounded-md transition w-full"
    >
      Back to homepage
    </a>
  </div>
</Card>
