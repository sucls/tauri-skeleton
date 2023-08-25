<script lang="ts">
    import { storeSubMenusHidden } from '$lib/stores/stores';

    import IonIcon from '$lib/icon/IonIcon.svelte';
	import Seacher from '$lib/components/Seacher/Seacher.svelte';
	import Toolbar from '$lib/components/Toolbar/Toolbar.svelte';
	import SiderMenu from '$lib/components/SiderMenu/SiderMenu.svelte';
    import { showModel } from '$lib/components/Modal/Model';
    import WriteEmail from '$lib/components/SiderMenu/WriteEmail.svelte';
	import { goto } from '$app/navigation';
	import { invoke } from '$lib/tools/tauri/tauri';

    // get by load
    export let data: any;
    
    const { emails } = data;

    /**
     * 写邮件
     */
    const writeEmail = ()=>{
        // 打开新窗口
        // invoke('plugin:window|open', { url: 'http://localhost:5173/email/write'});
        showModel('', { ref: WriteEmail })
    }

    /**
     * 刷新邮箱
     */
    const refreshEmail = ()=>{
        console.log('收邮件')
        invoke('plugin:email|batch_receive', { source: '409835152@qq.com'});
    }

</script>

<div class="flex-auto w-full h-full flex overflow-hidden">
    <div class="flex-none overflow-x-hidden overflow-y-auto bg-surface-50-900-token lg:w-auto border-r border-surface-500/30 {$storeSubMenusHidden? 'hidden':''} ">
        <!-- toolbar  -->
        <div class="toolbar">
            <Toolbar slotLead="space-x-4" background="o-transparent">
                <svelte:fragment slot="lead">
                    <IonIcon name="create" size="small" class="hover:variant-soft-primary" on:click={ writeEmail }/>
                    <IonIcon name="sync" size="small" class="hover:variant-soft-primary" on:click={ refreshEmail }/>
                    <!-- <i class="hover:variant-soft-primary">
                    </i> -->
                </svelte:fragment>
                <svelte:fragment slot="trail">
                    <IonIcon name="ellipsis-horizontal" size="small" class="hover:variant-soft-primary"/>
                </svelte:fragment>
            </Toolbar>
        </div>
        <!--  search  -->
        <div class="">
            <Seacher querySelection={ (input)=>[] }  inputClass="p-1 border"/>
        </div>
        <!-- SubMenus --> 
        <SiderMenu />
    </div>
    <!--  page content  -->
    <div class="overflow-y-auto flex-1 overflow-x-hidden flex flex-col ">
        <slot/>
    </div>
</div>