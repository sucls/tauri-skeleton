<script lang="ts">

    import Seacher from '$lib/components/Seacher/Seacher.svelte';
	import type { SearchResult } from '$lib/components/Seacher/Searcher';

	import type { TreeItem } from '$lib/components/Tree/Tree.'
	import Tree from '$lib/components/Tree/Tree.svelte';
	import IonIcon from '$lib/icon/IonIcon.svelte';
	import { TreeView, TreeViewItem } from '@skeletonlabs/skeleton';
    import type { ListItem } from '$lib/components/List/List'
	import List from '$lib/components/List/List.svelte';
    import PageContainer from '$lib/components/PageContainer/PageContainer.svelte';

    /*   Seacher  */

    const searchResult: SearchResult[] = [
		{ text: 'Vanilla', code: 'vanilla', meta: { healthy: false } },
		{ text: 'Chocolate', code: 'chocolate',  meta: { healthy: false } },
		{ text: 'Strawberry', code: 'strawberry', meta: { healthy: true } },
		{text: 'Neapolitan',code: 'neapolitan',	meta: { healthy: false }},
		{ text: 'Pineapple', code: 'pineapple',  meta: { healthy: true } },
		{ text: 'Peach', code: 'peach', meta: { healthy: true } }
	];

    const queryOptions = (input:string) =>{
        if( !input ){
            return [];
        }
        // 后台查询数据
        return searchResult;
    }

    /*  Tree  */
    const treeItems: TreeItem[] = [
        { 
            text: 'junny@google.com', icon:'logo-chrome', expanded: false,
            children: [ {text:'收件箱'},{text:'未读邮件'},{text:'红旗邮件'},{text:'草稿箱'} ] 
        }
    ]

    /*  List  */
    const listItem: ListItem[] = [
		{ avatar: 'YOErFW8AfkI', label: 'Item A', icon:'heart', description:'Content of Item A' },
		{ avatar: 'z_X0PxmBuIQ', label: 'Item B', icon:'heart', description:'Content of Item A' },
		{ avatar: '8vKVlNIbAc4', label: 'Item C', icon:'heart', description:'Content of Item A' }
	];
    const listItem2: ListItem[] = [
		{ label: '所有收件箱', icon:'folder', children: [ { label: 'su_chunlong@126.com', icon:'heart'},{ label: '409835152@qq.com', icon:'document'} ]},
		{ label: '所有未读', icon:'folder', children: [ { label: 'su_chunlong@126.com', icon:'heart'},{ label: '409835152@qq.com', icon:'document'}]},
	];

</script>

<PageContainer>
	<div class="space-y-5 flex-1">
		<h1 class="h1">Components</h1>
        <div class="flex flex-col">
            <h5 class="title"> AutoComplete </h5>
            <div class="flex flex-1">
                <div  class=" p-10 flex-1">
                    <Seacher querySelection={queryOptions} placeholder="请输入查询条件"/>
                </div>
                                
                <div  class=" p-10 flex-1">
                    <Seacher querySelection={queryOptions}/>
                </div>
            </div>

            <h5 class="title"> Tree </h5>
            <div class=" flex flex-1">
                <div  class=" p-10 flex-1">
                    <Tree {treeItems}/>
                </div>
                <div  class=" p-10 flex-1">
                    <Tree {treeItems} popup={true}/>
                </div>
            </div>

            <h5 class="title"> List </h5>
            <div class=" flex flex-1">
                <div  class=" p-10 flex-1">
                    <List items = {listItem} type='ul'/>
                </div>
                <div  class=" p-10 flex-1">
                    <List items = {listItem} type='ol'/>
                </div>
            </div>
            <div class=" flex flex-1">
                <div  class=" p-10 flex-1">
                    <List items = {listItem} type='desc'/>
                </div>
                <div  class=" p-10 flex-1">
                    <List items = {listItem} type='nav'/>
                </div>
            </div>

            <div class=" flex flex-1">
                <div  class=" p-10 flex-1">
                    <List items = {listItem2} type='tree'/>
                </div>
            </div>
        </div>
    </div>
</PageContainer>


<style lang="postcss">
    .title{
        @apply border-b-2 border-orange-600 pb-2;
    }
</style>

