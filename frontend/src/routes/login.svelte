<script context="module" lang="ts">
  export function load({ url }) {
    const next = url.searchParams.get("continue");
    if (!next) return { status: 400, error: "Invalid parameters" };

    return {
      props: {
        nextService: "fluxsearch",
      },
    };
  }
</script>

<script lang="ts">
  import Card from "../components/Card.svelte";
  import HeaderIcon from "eva-icons/outline/svg/lock-outline.svg";

  enum LoginStage {
    Identity,
    Password,
  }

  let stage: LoginStage = LoginStage.Identity;
  let loading = false;

  let data = {
    username: "",
  };
  export let nextService: string;
</script>

<Card>
  <div
    class:hidden={!loading}
    class="absolute h-2 w-full -mt-8 -ml-8 bg-gradient-to-r from-teal-900 to-teal-600
      animate-pulse duration-75 transition opacity-80 rounded-tl-md rounded-tr-md"
  />
  <form class:opacity-30={loading} class:pointer-events-none={loading} class="relative transition"
  on:submit|preventDefault={() => {}}>
    <div class="text-center mb-6">
      <HeaderIcon width="50" class="mx-auto mb-4 fill-teal-700" />

      {#if stage == LoginStage.Identity}
        <h1 class="text-xl font-medium">Sign in to continue</h1>
        <p class="text-sm text-stone-500 dark:text-stone-400 font-light">
          <span class="font-medium">{nextService}</span> needs authentication
        </p>
      {:else if stage == LoginStage.Password}
        <h1 class="text-xl font-medium">Hi, {data.username}</h1>
        <a
          href="#switch"
          class="block text-xs text-teal-700 underline dark:text-stone-400
        hover:text-teal-900 transition font-light"
          on:click|preventDefault={() => (stage = LoginStage.Identity)}
        >
          Switch Account
        </a>
      {/if}
    </div>
    <div class="mb-6">
      {#if stage == LoginStage.Identity}
        <input
          autofocus
          type="text"
          class="w-full px-3 py-2 font-light border border-stone-300 rounded-lg shadow-inner
          focus:shadow-teal-700/20 hover:border-teal-500 focus:border-teal-700 focus:transition
          outline-none"
          placeholder="Email or Username"
          bind:value={data.username}
        />
      {:else if stage == LoginStage.Password}
        <input
          autofocus
          type="password"
          class="w-full px-3 py-2 font-light border border-stone-300 rounded-lg shadow-inner
          focus:shadow-teal-700/20 hover:border-teal-500 focus:border-teal-700 focus:transition
          outline-none"
          placeholder="Password"
        />

        <!--<div class="mt-2">
          <label class="text-sm text-stone-600 cursor-pointer">
            <input
              class="form-check-input appearance-none h-4 w-4 border border-stone-300 rounded-sm shadow-inner
            checked:bg-teal-700 checked:border-teal-800 focus:outline-none transition duration-200
            mt-1 align-top bg-no-repeat bg-center bg-contain mr-1 cursor-pointer rounded"
              type="checkbox"
            />
            Remember me
          </label>
        </div>-->
      {/if}
    </div>
    <div class="flex flex-row justify-between items-center text-sm">
      {#if stage == LoginStage.Identity}
        <a
          href="/register"
          class="text-teal-800 -ml-2 p-2 hover:bg-neutral-100 hover:text-teal-700 rounded-md transition"
          >Create an account</a
        >
      {:else if stage == LoginStage.Password}
        <a
          href="/forget"
          class="text-teal-800 -ml-2 p-2 hover:bg-neutral-100 hover:text-teal-700 rounded-md transition"
          >Forgot password?</a
        >
      {/if}
      <button
        type="submit"
        class="px-6 py-2 text-white bg-teal-700 shadow hover:shadow-lg hover:bg-teal-800
      active:bg-teal-900 rounded-md transition"
        on:click={() => {
          loading = true;
          setTimeout(() => {
            stage = LoginStage.Password;
            loading = false;
          }, 1000);
        }}>Next</button
      >
    </div>
    <!--<footer class="w-full mt-6 text-neutral-500 text-center text-xs font-light">
      &copy; 2022, flux.ci
    </footer>-->
  </form>
</Card>
