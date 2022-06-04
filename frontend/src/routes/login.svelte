<script context="module" lang="ts">
  import type { Load } from "@sveltejs/kit";
  import { transport, metadata } from "$lib/grpc";
  import { LoginServiceClient } from "../../proto/login.client";

  export const load: Load = async ({ url, session }) => {
    const continueUrl = url.searchParams.get("continue");
    const backUrl = url.searchParams.get("back");
    if (!continueUrl) return { status: 400, error: "Invalid parameters" };

    const client = new LoginServiceClient(await transport());
    const data = (
      await client.initiate(
        {
          continueUrl,
          backUrl,
        },
        {
          ...metadata(session),
        }
      )
    ).response;

    const props = {
      currentHost: url.host,
      backUrl,
    };

    const defaultError = { error: "Unexpected server response" };

    switch (data.response.oneofKind) {
      case "result":
        const { result } = data.response.result;
        switch (result.oneofKind) {
          case "data":
            return { props: { ...result.data, ...props } };

          case "redirectUrl":
            return { status: 307, redirect: result.redirectUrl };

          default:
            return defaultError;
        }

      case "error":
        let status = data.response.error.httpResponseCode;
        if (status < 100) status = 500;

        return { status, error: data.response.error.message };

      default:
        console.error("Error", data);
        return defaultError;
    }
  };
</script>

<script lang="ts">
  import Card from "../components/Card.svelte";
  import HeaderIcon from "eva-icons/outline/svg/lock-outline.svg";
  import BackIcon from "eva-icons/outline/svg/arrow-back-outline.svg";
  import { fade, fly } from "svelte/transition";

  enum LoginStage {
    Identity,
    Password,
  }

  let stage: LoginStage = LoginStage.Identity;
  let loading = false;
  let height = 102;

  let data = {
    username: "",
  };

  export let currentHost: string;
  export let nextServiceName: string;
  export let backUrl: string | undefined;

  $: backUrlIsSameHost = new URL(backUrl).host === currentHost;
</script>

<svelte:head>
  <title>Login to {nextServiceName} - flux.ci</title>
</svelte:head>

<Card>
  {#if loading}
    <div
      transition:fade={{ duration: 200 }}
      class="absolute h-2 w-full -mt-8 -ml-8 bg-gradient-to-r from-teal-900 to-teal-600
        animate-pulse duration-75 opacity-80 rounded-tl-md rounded-tr-md"
    />
  {/if}
  <form
    class:opacity-30={loading}
    class:pointer-events-none={loading}
    class="relative transition"
    on:submit|preventDefault={() => {}}
  >
    {#if backUrl && stage == LoginStage.Identity}
      <a
        rel={backUrlIsSameHost ? null : "external"}
        href={backUrl}
        class="absolute -ml-3 -mt-2"
        transition:fade|local
      >
        <BackIcon width="30" class="fill-stone-400 hover:fill-stone-500 transition" />
      </a>
    {/if}
    <div class="text-center mb-6">
      <HeaderIcon width="50" class="mx-auto mb-4 fill-teal-700" />

      {#if stage == LoginStage.Identity}
        <h1 class="text-xl font-medium">Sign in to continue</h1>
        <p class="text-sm text-stone-500 dark:text-stone-400 font-light">
          <span class="font-medium">{nextServiceName}</span> needs authentication
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
    <div class="relative transition-[height]" style="height:{height}px">
      {#if stage == LoginStage.Identity}
        <div
          class="absolute top-0 w-full"
          bind:clientHeight={height}
          in:fly|local={{ x: 200, delay: 400 }}
          out:fade|local
        >
          <div class="mb-6">
            <input
              autofocus
              type="text"
              class="w-full px-3 py-2 font-light border border-stone-300 rounded-lg shadow-inner
              focus:shadow-teal-700/20 hover:border-teal-500 focus:border-teal-700 transition
              outline-none"
              placeholder="Email or Username"
              bind:value={data.username}
            />
          </div>
          <div class="flex flex-row justify-between items-center text-sm">
            <a
              href="/register"
              class="text-teal-800 -ml-2 p-2 hover:bg-neutral-100 hover:text-teal-700 rounded-md transition"
              >Create an account</a
            >
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
        </div>
      {:else if stage == LoginStage.Password}
        <div
          class="absolute top-0 w-full"
          bind:clientHeight={height}
          in:fly|local={{ x: 200, delay: 400 }}
          out:fade|local
        >
          <div class="mb-6">
            <input
              autofocus
              type="password"
              class="w-full px-3 py-2 font-light border border-stone-300 rounded-lg shadow-inner
              focus:shadow-teal-700/20 hover:border-teal-500 focus:border-teal-700 transition
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
          </div>
          <div class="flex flex-row justify-between items-center text-sm">
            <a
              href="/forget"
              class="text-teal-800 -ml-2 p-2 hover:bg-neutral-100 hover:text-teal-700 rounded-md transition"
              >Forgot password?</a
            >
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
        </div>
      {/if}
    </div>
    <!--<footer class="w-full mt-6 text-neutral-500 text-center text-xs font-light">
      &copy; 2022, flux.ci
    </footer>-->
  </form>
</Card>
