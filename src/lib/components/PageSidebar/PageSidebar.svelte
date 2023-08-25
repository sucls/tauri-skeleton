<script lang="ts">
    import { page } from '$app/stores';

    import { popup } from '@skeletonlabs/skeleton';
    import type { PopupSettings } from '@skeletonlabs/skeleton';

	import IonIcon from "$lib/icon/IonIcon.svelte";
    import { Avatar, AppRail, AppRailAnchor, AppRailTile } from "@skeletonlabs/skeleton";

    let currSelect: string;
    const iconSize:number = 25;

    const picPopupFeatured: PopupSettings = {
        // Represents the type of event that opens/closed the popup
        event: 'click',
        // Matches the data-popup value on your popup element
        target: 'picPopupFeatured',
        // Defines which side of your trigger the popup will appear
        placement: 'bottom',
    };

    const settingPopup: PopupSettings = {
        event: 'click',
        target: 'settingPopup',
        placement: 'right',
        middleware:{
            offset: 15
        }
    };

    page.subscribe( (page)=>{
        let basePath: string = page.url.pathname.split('/')[1];
        currSelect = '/'+ basePath;
    } );

    $: listboxItemActive = (href: string) => ($page.url.pathname?.includes(href) ? 'bg-primary-active-token' : '');

</script>

<div class="grid grid-cols-[auto_1fr] h-full bg-surface-50-900-token border-r border-surface-500/30 {$$props.class??''}">

    <AppRail background="bg-transparent" border="border-r border-surface-500/30" width="w-14" spacing="space-y-0"
        regionLead=""  regionTrail="">

        <svelte:fragment slot="lead">
            <AppRailAnchor>
                <button class="bg:hidden" use:popup={picPopupFeatured}>
                    <Avatar fill="fill-token" width="w-10" class="ml-auto mr-auto" initials="SC"/>
                </button>
                <div class="card p-4 w-82 shadow-xl z-10" data-popup="picPopupFeatured">
                    <div class="flex">
                        <div class="flex-1">
                            <Avatar initials="SC" width="w-16"/>
                        </div>
						<div class="flex-initial text-left pl-2">
							<p class="font-bold">SUCL</p>
                            <p class="opacity-50"><span>公司</span><span>/部门</span><span>/职务</span></p>
                            <div class="flex gap-2 pt-2">
                                <small>手机</small>
                                <small>微信</small>
                            </div>
                            <div class="flex gap-4 pt-2">
                                <a class="btn btn-sm variant-soft" href="###">修改个人信息</a>
                                <a class="btn btn-sm variant-soft" href="###">邮箱管理</a>
                            </div>
						</div>
					</div>
					<div class="arrow bg-surface-100-800-token" />
                </div>
            </AppRailAnchor>

            <AppRailAnchor href={'/demo'} title="测试" selected={ currSelect == '/demo' }>
                <IonIcon size="{iconSize}" name="albums" active={currSelect == '/demo'}></IonIcon>
            </AppRailAnchor>
            
            <AppRailAnchor href={'/email'} title="邮件" selected={ currSelect == '/email' }>
                <IonIcon size="{iconSize}" name="mail" active={currSelect == '/email'}></IonIcon>
            </AppRailAnchor>

            <AppRailAnchor href={'/workspace'} title="我的空间" selected={currSelect == '/workspace'}>
                <IonIcon size="{iconSize}" name="cloud" active={currSelect == '/workspace'}></IonIcon>
            </AppRailAnchor>

            
            <AppRailAnchor href={'/contact'} title="通讯录" selected={currSelect == '/contact'}>
                <IonIcon size="{iconSize}" name="people" active={currSelect == '/contact'}></IonIcon>
            </AppRailAnchor>

            <AppRailAnchor href={'/calendar'} title="日历" selected={currSelect == '/calendar'}>
                <IonIcon size="{iconSize}" name="calendar-number" active={currSelect == '/calendar'}></IonIcon>
            </AppRailAnchor>
        </svelte:fragment>

        <svelte:fragment slot="trail">
            <AppRailTile group="" name="feedback" value="feedback" title="反馈">
                <div class="card p-2 w-42 shadow-xl z-10 absolute" data-popup="settingPopup">
                    <ul class="list-nav text-left">
                        <li><a href="###"><span class="flex-auto">常见问题</span></a></li>
                        <li><a href="###"><span class="flex-auto">问题反馈与建议</span></a></li>
                        <li><a href="###"><span class="flex-auto">参与调研</span></a></li>
                    </ul>
                </div>
                <button use:popup={settingPopup}>
                    <IonIcon size="{iconSize}" name="menu"></IonIcon>
                </button>
            </AppRailTile>

            <AppRailAnchor href="/setting" title="设置" selected={currSelect == '/setting'}>
                <IonIcon size="{iconSize}" name="settings" active={currSelect == '/setting'}></IonIcon>
            </AppRailAnchor>
        </svelte:fragment>

    </AppRail>

</div>