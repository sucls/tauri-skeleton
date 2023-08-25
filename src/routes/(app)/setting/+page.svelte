<script lang="ts">
    
    import PageContainer from "$lib/components/PageContainer/PageContainer.svelte";
    import List from "$lib/components/List/List.svelte";
	import type { ListItem } from "$lib/components/List/List";

    let items: ListItem[] = [
        {label: '大师号', href:"#account"},
        {label: '系统设置', href:"#system-setting"},
        {label: '邮件通知', href:"#email-notice"},
        {label: '邮箱设置', href:"#email-address"},
        {label: '邮件读写', href:"#email-rw"},
        {label: '日历', href:"#calendar"},
        {label: '快捷键', href:"#shortcut-key"},
        {label: '代理设置', href:"#proxy-setting"},
        {label: '安全锁', href:"#security-lock"},
        {label: '关于', href:"#about"},
    ];

    let forom: any;
</script>

<div class="flex-auto w-full h-full flex overflow-hidden">
    <div class="flex-none overflow-x-hidden overflow-y-auto border-r border-surface-500/30 w-[150px] my-5">
        <List {items} type="nav" itemClass="text-xs text-right"/>
    </div>
    <!--  page content  -->
    <div class="overflow-y-auto flex-1 overflow-x-hidden flex flex-col my-5">
        <div class="part">
            <label id="account" class="label" for="title">大师号</label>
            <div class="border border-surface-500/30 w-full flex">

            </div>
        </div>
        <div class="part">
            <label id="system-setting" class="label" for="title">系统设置</label>
            <div class="border border-surface-500/30 w-full flex">

            </div>
        </div>
        <div class="part">
            <label id="email-notice" class="label" for="title">邮件通知</label>
            <div class="border border-surface-500/30 w-full flex">

            </div>
        </div>
        <div class="part">
            <label id="about" class="label" for="notice">关于</label>
            <div class="border border-surface-500/30 w-full flex">

            </div>
        </div>
    </div>
</div>

<style lang="postcss">
    .part>div{
        @apply h-[300px]
    }
</style>