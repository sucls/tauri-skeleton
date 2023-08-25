<script lang="ts">
    import { page } from '$app/stores';
    import { assets, base } from '$app/paths';
    import { drawerStore, Avatar, type ModalSettings } from "@skeletonlabs/skeleton";
    

	import IonIcon from "$lib/icon/IonIcon.svelte";
    import { menuNavLinks } from '$lib/Menu';
	import { findAvatarInitials } from "@/util/StringUtils";
    import { loadEmails, loadEmailFolders } from '@/service/EmailService';
    import type { Email, EmailFolder, Folder, ExpandedAble } from '@/types';
    import { showModel } from '$lib/components/Modal/Model';
    import LoginModal from './LoginModal.svelte';
	import { onMount } from 'svelte';

    let emails: (Email & ExpandedAble) [] = [];

    let emailFolders: EmailFolder[] = [];

    onMount(async ()=>{
        emails = await loadEmails();
        emailFolders = await loadEmailFolders( emails as Email[]);
    })

    /**
     * 
     */
    interface FolderMap{
        [key: string]: Folder[];
    }

    let currPath: keyof typeof menuNavLinks | undefined = undefined;
    let emaliAddressFoldersMap: FolderMap;// {[key: string]: EmailFolder[]};

    const addEmail = (evt: MouseEvent)=>{
        showModel( '', { ref: LoginModal }, 
        {
            image: `${assets}/logo.png`, 
            response: (r: any)=>{
                loadEmails().then(data=>{
                    emails = data;
                });
                loadEmailFolders().then(data=>{
                    emailFolders = data;
                })
            }
        } as ModalSettings );
    }

    // 监听url变更
    page.subscribe( (page)=>{
        let basePath: string = page.url.pathname.split('/')[1];
        currPath = basePath;
    } );

    $: submenus = menuNavLinks[currPath ?? '/'];

    $: listboxItemActive = (href: string) => ($page.url.pathname?.includes(href) ? 'bg-primary-active-token' : '');

    $: emaliAddressFoldersMap = emailFolders && emailFolders.reduce( (pre: FolderMap, curr)=>{
        pre[ curr.address ] = curr.folders;
        return pre;
    }, {});
</script>

<section class="px-0 py-2">
    {#each submenus as segment, i}
        <!-- 标题 -->
        <div class="flex justify-between px-3">
            <div class="flex flex-1">
                {#if segment.icon}
                <IonIcon name={segment.icon} class="px-1"></IonIcon>
                {/if}
                <p class="font-thin text-sm ">{segment.title}</p>
            </div>
            <!--    -->
            {#if segment.custom}
                <IonIcon name="add-circle" class="px-1" title="添加" on:click={ addEmail }></IonIcon>
            {/if}
        </div>
        <!-- 子菜单 -->
        <nav class="list-nav text-sm mt-1">
            <ul>
                <!--  邮箱文件夹  -->
                {#if segment.custom}
                    {#each emails as { address, kind, expanded } }
                        <li on:keypress on:click={drawerStore.close}>
                            <a href="###" class={listboxItemActive('#')} data-sveltekit-preload-data="hover">
                                <IonIcon name={expanded?'chevron-down':'chevron-forward'} on:click={ ()=>{expanded=!expanded} }/>
                                <Avatar fill="fill-token" width="w-5" class="" initials={findAvatarInitials(kind, address)}/>
                                <span class="flex-auto" style="margin-left: 5px;">{address}</span>
                                <!-- {#if badge}<span class="badge variant-filled-secondary">{badge}</span>{/if} -->
                            </a>
                            <!--  todo 加载邮箱文件夹  -->
                            {#if emaliAddressFoldersMap[address]}
                            <nav class="list-nav text-sm mt-1 pl-5 {expanded?'':'hidden'}">
                                <ul>
                                    {#each emaliAddressFoldersMap[address] as { name, icon } }
                                        <li on:keypress on:click={drawerStore.close}>
                                            <a href="###" class={listboxItemActive('#')} data-sveltekit-preload-data="hover">
                                                <IonIcon name={icon?icon:'folder'}/>
                                                <span class="flex-auto" style="margin-left: 5px;">{name}</span>
                                            </a>
                                        </li>
                                    {/each}
                                </ul>
                            </nav>
                            {/if}
                        </li>
                    {/each}
                {:else} 
                    <!-- 文件夹邮箱  -->
                    {#each segment.list as { label, icon, href, expanded, badge, keywords }}
                        <li on:keypress on:click={drawerStore.close}>
                            <a {href} class={listboxItemActive(href)} data-sveltekit-preload-data="hover" >
                                {#if currPath == 'email'}
                                <IonIcon name={expanded?'chevron-down':'chevron-forward'} on:click={ ()=>{expanded=!expanded} }/>
                                {/if}
                                {#if icon }
                                <IonIcon name={icon} style="margin-left:5px;"></IonIcon>
                                {/if}
                                <span class="flex-auto" style="margin-left: 5px;">{label}</span>
                                <!-- {#if badge}<span class="badge variant-filled-secondary">{badge}</span>{/if} -->
                            </a>
                            <!--  todo 加载邮箱地址  -->
                            {#if currPath == 'email'}
                            <nav class="list-nav text-sm mt-1 pl-5 {expanded?'':'hidden'}">
                                <ul>
                                    {#each emails as { address, kind } }
                                        <li on:keypress on:click={drawerStore.close}>
                                            <a href="###" class={listboxItemActive('#')} data-sveltekit-preload-data="hover">
                                                <Avatar fill="fill-token" width="w-5" class="" initials={findAvatarInitials(kind, address)}/>
                                                <span class="flex-auto" style="margin-left: 5px;">{address}</span>
                                                <!-- {#if badge}<span class="badge variant-filled-secondary">{badge}</span>{/if} -->
                                            </a>
                                        </li>
                                    {/each}
                                </ul>
                            </nav>
                            {/if}
                        </li>
                    {/each}
                {/if}
            </ul>
        </nav>
        <!-- Divider -->
        {#if i + 1 < submenus.length}<hr class="!my-6 opacity-50" />{/if}
    {/each}
</section>