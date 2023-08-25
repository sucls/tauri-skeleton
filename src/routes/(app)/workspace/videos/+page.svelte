<script lang="ts">
	// DocShell
	import DocsShell from '$lib/layouts/DocsShell/DocsShell.svelte';
	import { DocsFeature, type DocsShellSettings } from '$lib/layouts/DocsShell/types';

	// Data
	import { movies } from './movies';

	// Docs Shell
	const settings: DocsShellSettings = {
		feature: DocsFeature.Element,
		name: '视频资源',
		description: ''
	};

	// Multi-Column ---

	let elemMovies: HTMLDivElement;

	function multiColumnLeft(): void {
		let x = elemMovies.scrollWidth;
		if (elemMovies.scrollLeft !== 0) x = elemMovies.scrollLeft - elemMovies.clientWidth;
		elemMovies.scroll(x, 0);
	}

	function multiColumnRight(): void {
		let x = 0;
		// -1 is used because different browsers use different methods to round scrollWidth pixels.
		if (elemMovies.scrollLeft < elemMovies.scrollWidth - elemMovies.clientWidth - 1) x = elemMovies.scrollLeft + elemMovies.clientWidth;
		elemMovies.scroll(x, 0);
	}
</script>

<DocsShell {settings}>
	<!-- Slot: Usage -->
	<svelte:fragment slot="usage">
		<section class="space-y-4">
			<h2 class="h2">电影</h2>
			<div class="grid grid-cols-[auto_1fr_auto] gap-4 items-center">
				<!-- Button: Left -->
				<button type="button" class="btn-icon variant-filled" on:click={multiColumnLeft}>
					<i class="fa-solid fa-arrow-left" />
				</button>
				<!-- Carousel -->
				<div bind:this={elemMovies} class="snap-x snap-mandatory scroll-smooth flex gap-2 pb-2 overflow-x-auto">
					{#each movies as movie}
						<a href={movie.url} target="_blank" class="shrink-0 w-[28%] snap-start">
							<img
								class="rounded-container-token hover:brightness-125"
								src={movie.imageUrl}
								alt={movie.name}
								title={movie.name}
								loading="lazy"
							/>
						</a>
					{/each}
				</div>
				<!-- Button-Right -->
				<button type="button" class="btn-icon variant-filled" on:click={multiColumnRight}>
					<i class="fa-solid fa-arrow-right" />
				</button>
			</div>
		</section>
	</svelte:fragment>
</DocsShell>