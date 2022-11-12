<script lang="ts">
	import '../app.css';
	import Navbar from '$lib/components/navbar.svelte';
</script>

<svelte:head>
	<title>Pollme</title>
</svelte:head>

<Navbar />
<slot />
