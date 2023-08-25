<script lang='ts'>
	// Dependency: Floating UI
	import { Modal, popup, storePopup } from '@skeletonlabs/skeleton';
	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	// The ordering of these imports is critical to your app working properly
	import '@skeletonlabs/skeleton/themes/theme-seafoam.css';
	// If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';

	import { AppShell } from '@skeletonlabs/skeleton';

	import PageSidebar from '$lib/components/PageSidebar/PageSidebar.svelte';

	import type { MenuItem } from '$lib/components/ContextMenu/ContextMenu';
	import ContextMenu from '$lib/components/ContextMenu/ContextMenu.svelte';
	import { storeContextMenuProps, globalStore } from '$lib/stores/stores';

	//
	let menuItems: MenuItem[] = [
		{id: '1', label: '刷新', name: 'refresh'},
		{id: '2', label: '返回', name: 'goback'},
		{id: '3', label: '保存', name: 'save'},
	];
	const handleContextmenu = ( event: MouseEvent ) => {
		console.log( '点击右键', event )
		event.preventDefault();

		// 基于当前环境构建
		$storeContextMenuProps.menuItems = menuItems;
		$storeContextMenuProps.position = {x: event.clientX, y: event.clientY}
		$storeContextMenuProps.visiable = true;
	}

	// 控制左侧边栏
	$: slotSidebarLeft = 'bg-surface-50-900-token lg:w-auto';
</script>

<svelte:window on:contextmenu={ handleContextmenu } on:click={ ()=>$storeContextMenuProps.visiable = false }/>

<ContextMenu/>

<Modal/>

<svelte:head>

</svelte:head>

<!-- App Shell -->
<AppShell {slotSidebarLeft} regionPage="overflow-y-auto" slotPageContent="h-full" slotFooter="bg-black p-4">
	<!-- Header -->
	<svelte:fragment slot="header">
		
	</svelte:fragment>

	<!-- Sidebar (Left) -->
	<svelte:fragment slot="sidebarLeft">
		<!-- lg:grid  基于@media实现侧边栏的隐藏与展示 ，记得加上hidden-->
		<PageSidebar class="lg:grid overflow-hidden {$globalStore.hiddenSideBar?'hidden':''}"/>
	</svelte:fragment>

	<!-- Page Content -->
	<slot />

	<!-- Page Footer -->
	<svelte:fragment slot="pageFooter">
		
	</svelte:fragment>
</AppShell>

