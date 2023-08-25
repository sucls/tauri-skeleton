<script lang="ts">
    
    import { TreeViewItem } from "@skeletonlabs/skeleton";

    import IonIcon from "$lib/icon/IonIcon.svelte";
    import type { TreeItem } from "./Tree";

    export let treeItems: TreeItem[];

    export let popup: boolean;

    let level;
</script>

{#if treeItems}
    {#each treeItems as { text, icon, expanded, children }}
        <TreeViewItem open={expanded}
            class="{popup?'relative':''}" regionChildren="{popup?'absolute w-full p-1 m-0 bg-secondary-50':''}"
            classesChildren = "{children && children.length?'':'hidden'} {$$props.classesChildren??''}">
            <svelte:fragment slot="lead" >
                {#if icon}
                    <IonIcon name="logo-chrome">{text}</IonIcon>
                {:else}
                    {text}
                {/if}
            </svelte:fragment>
            <!-- 图标问题 -->
            <svelte:fragment slot="children">
            {#if children}
                <svelte:self treeItems = {children} />
            {/if}
            </svelte:fragment>
        </TreeViewItem>
    {/each} 
{/if}

<style lang="postcss">

</style>