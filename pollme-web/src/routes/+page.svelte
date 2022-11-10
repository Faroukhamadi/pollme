<script lang="ts">
	import Poll from '$lib/components/poll.svelte';
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

{#each data.posts as post}
	<Poll {post} />
{/each}
