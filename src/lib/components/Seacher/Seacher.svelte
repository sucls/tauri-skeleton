<script lang="ts">
    import { onMount } from 'svelte';

	import type { AutocompleteOption, PopupSettings } from '@skeletonlabs/skeleton';
	import { Autocomplete, popup } from '@skeletonlabs/skeleton';

    import type { SearchResult } from './Searcher';
	
    /**
    * 根据输入值查询结果
    */
    export let querySelection: (key:string)=>SearchResult[];

    export let placeholder: string = '搜索';

    export let inputClass: string = 'p-1';

    export let autocompleteClass: string = '';

	let searchValue: string = '';

    const targetId: string = crypto.randomUUID() + '-popupAutocomplete';

	let popupSettings: PopupSettings = {
		event: 'focus-click',
		target: targetId,
		placement: 'bottom'
	};

    /**
     * 下拉选项
     */
    let options: AutocompleteOption[];

    let style:string;
    let inputElem:HTMLElement;
    let inputWidth: number;

    onMount( ()=>{
        inputWidth = inputElem.clientWidth;
    })

	function onSelection(event: any): void {
		searchValue = event.detail.label;
	}

    function onKeypress(event: any): void{
        console.log( 'keypress', event )   
    }

    function onClick(event: any): void{
        // console.log( 'click', event )
        if( !searchValue ){
            // 不打开选中框
            event.stopPropagation();
        }else{

        }
    }

    /**
     * 转换
     * @param result
     */
    function convertSearchResult(result: SearchResult[]): AutocompleteOption[]{
        let options: AutocompleteOption[] = [];
        if( result && result.length ){
            result.forEach( res =>{
                options.push(
                    { label: res.code, value: res.text, keywords: res.meta && res.meta.keywords, meta: {... res.meta } },
                )
            } )
        }
        return options;
    }

    // 响应式
    $: options = convertSearchResult( querySelection(searchValue) );
    $: {
        let styles:string[] = [`width: ${inputWidth}px`];
        if( !searchValue ){
            styles.push('display:none');
        }
        // styles.push('width:'+ inputElem.clientWidth )
        style = styles.join(';')
    }
        
</script>

<div class="email-searcher p-2 {$$props.class??''}">
    <!--  on:click={onClick} on:keypress={onKeypress} -->
	<input
        bind:this={inputElem}
		class = "input autocomplete w-full {inputClass??''}"
		type = "search"
		name= "autocomplete-search"
		bind:value= {searchValue}
		placeholder = {placeholder}
		use:popup = {popupSettings}
        
	/>
	<div data-popup={targetId} class="card w-full max-w-sm max-h-48 p-4 overflow-y-auto {autocompleteClass}" {style}>
		<Autocomplete
            emptyState = '没有找到结果'
			bind:input = {searchValue}
			bind:options = {options}
			on:selection = {onSelection}
            on:click = {onClick}
		/>
	</div>
</div>

<style>
    .email-searcher{
        font-size: .5em;
    }
    .searcher input{
        margin: 0 auto;
    }
    .email-searcher ::-webkit-input-placeholder{
        font-size: .8em;
        padding-left: 5px;
    }
</style>