<script lang="ts">
	import type { PageServerData } from './$types';
	import Poll from '$lib/components/poll.svelte';

	export let data: PageServerData;
</script>

{#each data.posts as post}
	<Poll {post} />
{/each}
