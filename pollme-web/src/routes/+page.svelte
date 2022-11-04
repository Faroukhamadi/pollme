<script lang="ts">
	import Poll from '$lib/components/poll.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

{#each data.posts as post}
	<Poll {post} />
{/each}
