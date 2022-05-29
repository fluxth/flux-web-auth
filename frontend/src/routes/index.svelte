<script lang="ts">
  import Card from '../components/Card.svelte';
  import HeaderIcon from 'eva-icons/outline/svg/person-delete-outline.svg';
</script>

<Card>
  <div class="text-center mb-6">
    <HeaderIcon width="50" class="mx-auto mb-4 fill-stone-400" />
    <h1 class="text-xl font-medium">You are not logged in</h1>
    <p class="text-stone-500 dark:text-stone-400 font-light">
      No active session detected
    </p>
  </div>
  <div class="text-sm text-center">
    <a href="/login?continue=a" class="block px-6 py-2 text-white bg-teal-700 shadow hover:shadow-lg hover:bg-teal-800 active:bg-teal-900 rounded-md transition w-full">
      Sign in
    </a>
  </div>
</Card>
