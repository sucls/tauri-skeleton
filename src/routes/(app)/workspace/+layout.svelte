<script lang="ts">

    import { page } from '$app/stores';

	import Seacher from "$lib/components/Seacher/Seacher.svelte";
	import IonIcon from "$lib/icon/IonIcon.svelte";
	import { storeSubMenusHidden } from "$lib/stores/stores";

    const submenus: any[] = [
        {
            title: '全部文件',
            icon: 'cube',
            list: [
                { href:'/workspace/files', label:'文档', keywords:'', icon: 'document' },
                { href:'/workspace/pictures', label:'图片', keywords:'', icon: 'image' },
                { href:'/workspace/musics', label:'音乐', keywords:'', icon:'musical-notes' },
                { href:'/workspace/videos', label:'视频', keywords:'', icon:'videocam' },
                { href:'/workspace/packages', label:'压缩包', keywords:'', icon:'library' },
                { href:'/workspace/others', label:'其他', keywords:'', icon: 'ellipsis-horizontal-circle' },
            ]
        },
        {
            title: '任务列表',
            icon: '',
            list: [  ]
        },
    ];

    $: listboxItemActive = (href: string) => ($page.url.pathname?.includes(href) ? 'bg-primary-active-token' : '');

</script>

<div class="flex-auto w-full h-full flex overflow-hidden">
    <div class="flex-none overflow-x-hidden overflow-y-auto bg-surface-50-900-token lg:w-auto border-r border-surface-500/30 {$storeSubMenusHidden? 'hidden':''} ">
        <!--  search  -->
        <div class="">
            <Seacher querySelection={ (input)=>[] }  inputClass="p-1 border"/>
        </div>
        <!-- SubMenus --> 
        <section class="px-0 py-2">
            {#each submenus as segment, i}
                <!-- 标题 -->
                <div class="flex justify-between px-3">
                    <div class="flex flex-1">
                        {#if segment.icon}
                        <IonIcon name={segment.icon} class="px-1"></IonIcon>
                        {/if}
                        <p class="font-thin text-sm pl-1">{segment.title}</p>
                    </div>
                </div>
                <nav class="list-nav text-sm mt-1">
                    <ul>
                        {#each segment.list as { label, icon, href, expanded, badge, keywords }}
                        <li on:keypress class="pl-6">
                            <a {href} class={listboxItemActive(href)} data-sveltekit-preload-data="hover" >
                                {#if icon }
                                <IonIcon name={icon}></IonIcon>
                                {/if}
                                <span class="flex-auto" style="margin-left: 5px;">{@html label}</span>
                                <!-- {#if badge}<span class="badge variant-filled-secondary">{badge}</span>{/if} -->
                            </a>
                        </li>
                        {/each}
                    </ul>
                </nav>
                <!-- Divider -->
                {#if i + 1 < submenus.length}<hr class="!my-6 opacity-50" />{/if}
            {/each}
        </section>
    </div>
    <!--  page content  -->
    <div class="overflow-y-auto flex-1 overflow-x-hidden flex flex-col ">
        <slot/>
    </div>
</div>