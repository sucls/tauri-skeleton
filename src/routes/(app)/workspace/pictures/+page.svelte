<script lang="ts">
	// DocShell
	import DocsShell from '$lib/layouts/DocsShell/DocsShell.svelte';
	import { DocsFeature, type DocsShellSettings } from '$lib/layouts/DocsShell/types';

	// Docs Shell
	const settings: DocsShellSettings = {
		feature: DocsFeature.Element,
		name: '图片资源',
		description: ''
	};

	// Carousel ---

	let elemCarousel: HTMLDivElement;
	const unsplashIds = ['vjUokUWbFOs', '1aJuPtQJX_I', 'Jp6O3FFRdEI', 'I3C_eojFVQY', 's0fXOuyTH1M', 'z_X0PxmBuIQ'];

	function carouselLeft(): void {
		const x =
			elemCarousel.scrollLeft === 0
				? elemCarousel.clientWidth * elemCarousel.childElementCount // loop
				: elemCarousel.scrollLeft - elemCarousel.clientWidth; // step left
		elemCarousel.scroll(x, 0);
	}

	function carouselRight(): void {
		const x =
			elemCarousel.scrollLeft === elemCarousel.scrollWidth - elemCarousel.clientWidth
				? 0 // loop
				: elemCarousel.scrollLeft + elemCarousel.clientWidth; // step right
		elemCarousel.scroll(x, 0);
	}

	function carouselThumbnail(index: number) {
		elemCarousel.scroll(elemCarousel.clientWidth * index, 0);
	}
</script>

<DocsShell {settings}>
	<!-- Slot: Usage -->
	<svelte:fragment slot="usage">
		<section class="space-y-4">
			<div class="card p-4 grid grid-cols-[auto_1fr_auto] gap-4 items-center">
				<!-- Button: Left -->
				<button type="button" class="btn-icon variant-filled" on:click={carouselLeft}>
					<i class="fa-solid fa-arrow-left" />
				</button>
				<!-- Full Images -->
				<div bind:this={elemCarousel} class="snap-x snap-mandatory scroll-smooth flex overflow-x-auto">
					{#each unsplashIds as unsplashId}
						<img
							class="snap-center w-[1024px] rounded-container-token"
							src="https://source.unsplash.com/{unsplashId}/1024x768"
							alt={unsplashId}
							loading="lazy"
						/>
					{/each}
				</div>
				<!-- Button: Right -->
				<button type="button" class="btn-icon variant-filled" on:click={carouselRight}>
					<i class="fa-solid fa-arrow-right" />
				</button>
			</div>
			<!-- Thumbnails -->
			<div class="card p-4 grid grid-cols-6 gap-4">
				{#each unsplashIds as unsplashId, i}
					<button type="button" on:click={() => carouselThumbnail(i)}>
						<img
							class="rounded-container-token"
							src="https://source.unsplash.com/{unsplashId}/256x256"
							alt={unsplashId}
							loading="lazy"
						/>
					</button>
				{/each}
			</div>
		</section>
	</svelte:fragment>
</DocsShell>