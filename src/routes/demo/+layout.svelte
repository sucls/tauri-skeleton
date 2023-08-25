<div class="h-full flex flex-row">
	<section class="space-y-4 w-[200px] h-full border-r border-surface-500/30 py-2">
		<!-- Nav List -->
		<nav class="list-nav text-sm">
			<ul>
				<li>
					<a href="/demo/icon" class="" data-sveltekit-preload-data="hover">
						<span class="flex-auto">icon</span>
					</a>
					<a href="/demo/comps" class="" data-sveltekit-preload-data="hover">
						<span class="flex-auto">component</span>
					</a>
					<a href="/demo/tauri" class="" data-sveltekit-preload-data="hover">
						<span class="flex-auto">tauri</span>
					</a><a href="/demo/email" class="" data-sveltekit-preload-data="hover">
						<span class="flex-auto">email</span>
					</a>
					<a href="/demo/datasource" class="" data-sveltekit-preload-data="hover">
						<span class="flex-auto">datasource</span>
					</a>
				</li>
			</ul>
		</nav>
	</section>
	<!--  -->
	<div class="flex-1 flex">
		<slot />
	</div>
</div>

<style lang="postcss">
	ul > li > a {
		@apply rounded-none border-b border-b-teal-400;
	}
	ul > li > a:hover {
		@apply bg-emerald-400;
	}
</style>
