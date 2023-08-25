import { writable, type Writable } from 'svelte/store';
import { localStorageStore } from '@skeletonlabs/skeleton';
import type { ContextMenuProps } from '$lib/components/ContextMenu/ContextMenu';

// Svelte Writable Stores ---

// Set within the root layout, set TRUE if served in Vercel production mode
export const storeVercelProductionMode: Writable<boolean> = writable(false);

// Local Storage Stores ---

// Persists select preset theme
export const storeTheme: Writable<string> = localStorageStore('storeTheme', 'skeleton');

// Persists the tab selection for the user's preferred onboarding method
export const storeOnboardMethod: Writable<string> = localStorageStore('storeOnboardMethod', 'cli');

//
export const storeSubMenusHidden: Writable<boolean> = writable(false);

// 右键菜单
export const storeContextMenuProps: Writable<ContextMenuProps> = writable({ menuItems: [], position: {x:0,y:0}, visiable: false });


export const globalStore: Writable<any> = writable({ hiddenSideBar: true });
