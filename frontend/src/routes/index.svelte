<script lang="ts" context="module">
  import type { Load } from "@sveltejs/kit";
  import { transport, metadata } from "$lib/grpc";
  import { AuthenticationServiceClient } from "../../proto/authentication.client";

  export const load: Load = async ({ session }) => {
    const client = new AuthenticationServiceClient(await transport());
    const data = (
      await client.status(
        {},
        {
          ...metadata(session),
        }
      )
    ).response;

    switch (data.response.oneofKind) {
      case "result":
        return { props: data.response.result };
      case "error":
        return { error: data.response.error.message };
      default:
        console.error("Error", data);
        return { error: "Unexpected payload error" };
    }
  };
</script>

<script lang="ts">
  import Card from "../components/Card.svelte";
  import LoggedOutHeaderIcon from "eva-icons/outline/svg/person-delete-outline.svg";
  import LoggedInHeaderIcon from "eva-icons/outline/svg/person-done-outline.svg";
  import { page } from "$app/stores";

  export let loggedIn: boolean;
  export let displayName: string;

  $: nextUrl = encodeURIComponent($page.url.href);
</script>

<svelte:head>
  <title>flux.ci Authentication Service</title>
</svelte:head>

<Card>
  <div class="text-center mb-6">
    {#if loggedIn}
      <LoggedInHeaderIcon width="50" class="mx-auto mb-4 fill-teal-700" />
      <h1 class="text-xl font-medium">Hi, {displayName}</h1>
      <p class="text-stone-500 dark:text-stone-400 font-light">You are logged in!</p>
    {:else}
      <LoggedOutHeaderIcon width="50" class="mx-auto mb-4 fill-stone-400" />
      <h1 class="text-xl font-medium">You are not logged in</h1>
      <p class="text-stone-500 dark:text-stone-400 font-light">No active session detected</p>
    {/if}
  </div>
  <div class="text-sm text-center">
    {#if loggedIn}
      <a
        href="/settings"
        class="block px-6 py-2 mb-1 text-stone-600 bg-stone-50 border shadow hover:shadow-lg hover:bg-stone-100 active:bg-stone-200 rounded-md transition w-full"
      >
        Account Settings
      </a>
      <a
        href="/logout?continue={nextUrl}&back={nextUrl}"
        class="block px-6 py-2 text-white bg-red-700 shadow hover:shadow-lg hover:bg-red-800 active:bg-red-900 rounded-md transition w-full"
      >
        Sign out
      </a>
    {:else}
      <a
        href="/login?continue={nextUrl}&back={nextUrl}"
        class="block px-6 py-2 text-white bg-teal-700 shadow hover:shadow-lg hover:bg-teal-800 active:bg-teal-900 rounded-md transition w-full"
      >
        Sign in
      </a>
    {/if}
  </div>
</Card>
