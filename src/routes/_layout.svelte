<script lang="ts">
	import Nav from '../components/Nav.svelte';
	import Tailwind from "../components/Tailwind.svelte";
	export let segment: string;
</script>

<style>
	main {
		position: relative;
		max-width: 56em;
		background-color: white;
		padding: 2em;
		margin: 0 auto;
		box-sizing: border-box;
	}
</style>

<Tailwind />
<Nav {segment}/>

<main class="container mx-auto">
	<slot></slot>
</main>
