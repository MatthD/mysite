<style>
	h1, figure, p {
		text-align: center;
		margin: 0 auto;
	}

	h1 {
		font-size: 2.8em;
		text-transform: uppercase;
		font-weight: 700;
		margin: 0 0 0.5em 0;
	}

	figure {
		margin: 0 0 1em 0;
	}

	img {
		width: 100%;
		max-width: 400px;
		margin: 0 0 1em 0;
		border-radius: 0 0 25px 25px;
	}

	p {
		margin: 1em auto;
	}

	@media (min-width: 480px) {
		h1 {
			font-size: 4em;
		}
	}

	.mypicture{
		position: absolute;
		top:0;
		left: -100px;
		width: 100px;
		height: auto;
	}
</style>

<svelte:head>
	<title>Matthias Di Profile</title>
</svelte:head>

<h1>I'm a JS/TS passionate developper !</h1>

<figure class="mypicture">
	<img alt="Matthias Di face" src="https://avatars.githubusercontent.com/u/7901366?v=4">
	<figcaption>8 yo js dev 🦄</figcaption>
</figure>

<p><strong>Try editing this file (src/routes/index.svelte) to test live reloading.</strong></p>
