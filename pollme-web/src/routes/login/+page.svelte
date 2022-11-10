<script lang="ts">
	import LoginForm from '$lib/components/loginForm.svelte';
	import type { ActionData } from './$types';

	export let form: ActionData;
</script>

{#if form?.success}
	<div class="bg-green-500">
		{form?.success}
	</div>
{:else if form?.error}
	<div class="bg-red-400">{form?.error}</div>
{:else}
	<div class="bg-blue-400">Nothing to see here</div>
{/if}

<div class="hero min-h-screen bg-base-200 ">
	<div class="hero-content flex-col ">
		<div class="text-center lg:text-left">
			<h1 class="text-5xl font-bold">Login</h1>
		</div>
		<div class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
			<LoginForm />
		</div>
	</div>
</div>
