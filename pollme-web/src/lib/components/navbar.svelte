<script lang="ts">
	import { page } from '$app/stores';
</script>

<div class="navbar bg-base-100">
	<div class="flex-1">
		<h1>
			<a class="btn btn-ghost normal-case text-xl" data-sveltekit-prefetch href="/">Pollme</a>
		</h1>
	</div>
	{#if !$page.data.user}
		<div class="navbar-end">
			<a class="btn" data-sveltekit-prefetch href="/login">Login</a>
		</div>
	{/if}
</div>
