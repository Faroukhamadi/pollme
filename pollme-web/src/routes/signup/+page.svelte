<script lang="ts">
	import SignupForm from '$lib/components/signupForm.svelte';
	import type { ActionData } from './$types';

	export let form: ActionData;
</script>

{#if form?.error}
	<div class="bg-red-400">{form?.error}</div>
{:else}
	<div class="bg-blue-400">Nothing to see here</div>
{/if}

<div class="hero min-h-screen bg-base-200 ">
	<div class="hero-content flex-col ">
		<div class="text-center lg:text-left">
			<h1 class="text-5xl font-bold">Signup</h1>
		</div>
		<div class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
			<SignupForm />
		</div>
	</div>
</div>
