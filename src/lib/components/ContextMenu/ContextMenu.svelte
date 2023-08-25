<script lang="ts">
    import { page } from '$app/stores';

	import type { MenuItem , Position} from "./ContextMenu";

    import { storeContextMenuProps } from '$lib/stores/stores';

    /**
     * 
     */
    let visiable: boolean;
    /**
     * 
     */
    let menuItems: MenuItem[];
    /**
     * 
     */
    let position: Position;

    /**
     * 
    */
    storeContextMenuProps.subscribe( store=>{
        visiable = store.visiable;
        menuItems = store.menuItems;
        position = store.position;
    })

    export const onClick = (evt: Event)=>{
        console.log('点击菜单', evt)
    }

    $: baseClass = `card p-2 w-42 w-[200px] shadow-xl z-20 absolute ${visiable?'':'hidden'} ${$$props.class??''}`;

    $: baseStyle = [`left:${position?position.x:0}px`,`top:${position?position.y:0}px`].join(';');

</script>

<!--  data-popup={dataPopup} -->
<div class={baseClass} style={baseStyle}>
    {#if menuItems && menuItems.length}
    <ul class="list-nav text-left text-sm">
        {#each menuItems as item, i}
        <li on:click={ onClick } on:keydown on:keyup on:keypress>
            <a href="###"><span class="flex-auto">{item.label}</span></a>
        </li>
        {/each}
    </ul>
    {/if}
</div>