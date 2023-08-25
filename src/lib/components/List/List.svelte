<script lang="ts">
	import { Avatar } from "@skeletonlabs/skeleton";
	import type { ListItem, ListType } from "./List";
	import IonIcon from "$lib/icon/IonIcon.svelte";

    export let items: ListItem[];
    export let type: ListType = 'ul';

    export let itemClass: String = '';

    $: classes = `flex-auto ${itemClass??''}`
</script>

<section class="space-y-4 email-list">
    <!--  Unordered List  -->
    {#if type == 'ul'}
    <ul class="list">
        {#each items as item,i}
        <li>
            {#if item.icon}
                <Avatar src={item.icon} width="w-12" />
            {/if}
            <span class={classes}>{item.label}</span>
        </li>
        {/each}
    </ul>
    {/if}

    {#if type == 'ol'}
    <!-- Ordered  List -->
    <ol class="list">
        {#each items as item,i}
        <li>
            <span class="badge-icon p-4 variant-soft-primary">{i + 1}</span>
            <span class={classes}>{item.label}</span>
        </li>
        {/each}
    </ol>
    {/if}

    <!--  Description List  -->
    {#if type == 'desc'}
    <dl class="list-dl">
        {#each items as item}
        <div>
            {#if item.icon}
                <span class="badge-icon p-4 variant-soft-secondary"><IonIcon name={item.icon}/></span>    
            {/if}
            <span class={classes}>
                <dt class="font-bold">{item.label}</dt>
                <dd class="text-sm opacity-50">{item.description}</dd>
            </span>
        </div>
        {/each}
    </dl>
    {/if}

    <!-- Navigation List -->
    {#if type == 'nav'}
    <nav class="list-nav">
        <ul>
            {#each items as item}
            <li>
                <a href="{item.href}">
                    {#if item.icon}
                    <span class="badge-icon p-4 variant-soft-tertiary"><IonIcon name={item.icon}/></span>    
                     {/if}
                    <span class={classes}>
                        {item.label}
                    </span>
                </a>
            </li>
        {/each}
        </ul>
    </nav>
    {/if}

    
    <!-- Tree List -->
    {#if type == 'tree'}
    <nav class="list-nav list-tree {$$props.class??''}">
        <ul>
            {#each items as item}
            <li>
                <a href="{item.href}">
                    {#if item.children}
                        <IonIcon class="ml-2" name={item.expanded?'chevron-down':'chevron-forward'} on:click={()=>item.expanded=!item.expanded}/>
                    {/if}
                    
                    <span class="flex-auto">
                        {#if item.icon}
                        <IonIcon name={item.icon}/>
                    {/if}
                        {item.label}
                    </span>
                </a>
                {#if item.children}
                    <svelte:self type='tree' items={item.children} class=" pl-2.5 {item.expanded?'block':'hidden'}"/>
                {/if}
            </li>
        {/each}
        </ul>
    </nav>
    {/if}

</section>

<style lang="postcss">
    .list-tree ul li span, 
    .list-tree ul li a>*{
        @apply ml-2;
    }
</style>