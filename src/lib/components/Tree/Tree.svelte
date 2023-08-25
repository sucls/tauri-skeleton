<script lang="ts">
	import { TreeView } from "@skeletonlabs/skeleton";
    
	import type { TreeItem as Item } from "./Tree";
	import TreeItem from "./TreeItem.svelte";

    export let treeItems: Item[];
    /**
     * 是否弹窗
     */
    export let popup: boolean = false;

</script>

<div class="email-tree">
    <TreeView padding='py-2 px-2'>
        <TreeItem {treeItems} {popup}/>
    </TreeView>
</div>

<style lang="postcss">
    .email-tree{
        
    }
</style>