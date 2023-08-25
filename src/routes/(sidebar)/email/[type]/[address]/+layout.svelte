<script lang="ts">
    import { page } from '$app/stores';

    import { Avatar } from '@skeletonlabs/skeleton'

    import IonIcon from '$lib/icon/IonIcon.svelte';
	import Toolbar from '$lib/components/Toolbar/Toolbar.svelte';
	import { findFirstUp } from '@/util/StringUtils';

    export let data: any;

    // 来源于 +layout.ts load
    const { emails } = data;

    // 地址问题
    const resolvePageUrl = (id: string): string =>{
        const baseUrl = $page.url.pathname;
        if( $page.params.id ){
            let paths = baseUrl.split('/');
            return paths.slice(0, paths.length-1).concat([id]).join('/');
        }else{
            return [baseUrl, id].join('/');
        }
    }

    page.subscribe( (page)=>{
        let basePath: string = page.url.pathname.split('/')[1];
        console.log($page);
    } );

</script>

<div id="email" class="flex-auto w-full h-full flex overflow-hidden">
    <div class="flex-none overflow-x-hidden overflow-y-auto bg-surface-50-900-token lg:w-auto">
        <!-- toolbar  -->
        <div class="flex-none">
            <Toolbar slotLead="space-x-4" background="o-transparent">
                <svelte:fragment slot="lead">
                    <IonIcon name="funnel" size="small" class="hover:variant-soft-primary"/>
                </svelte:fragment>
            </Toolbar>
        </div>

        <!-- email list -->
        <section class="p-0 space-y-4 w-[220px]">
            <div class="w-full text-token p-0 space-y-4">
                <dl class="list-dl">
                    {#each emails as { id, subject, text, source }}
                        <div class="hover:variant-soft-primary rounded-none {$page.params.id === id? 'bg-primary-400':''}" style="border-radius: 0;">
                            <span class="self-start">
                                <Avatar fill="fill-token" width="w-10" class="ml-auto mr-auto" initials={ findFirstUp(source) }/>
                            </span>
                            <a class="flex-auto overflow-hidden" href="{resolvePageUrl(id)}">
                                <dt class="font">{source}</dt>
                                <dd class="text-sm opacity-80 truncate">{subject}</dd>
                                <dd class="text-sm opacity-50 truncate">{text}</dd>
                            </a>

                        </div>
                    {/each}
                </dl>
            </div>
        </section>
    </div>
    <!--  page content  -->
    <div class="overflow-y-auto flex-1 overflow-x-hidden flex flex-col">
        <slot/>
    </div>
</div>