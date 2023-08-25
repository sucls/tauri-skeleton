<script lang="ts">
    import '@wangeditor/editor/dist/css/style.css';

    import { createEditor, createToolbar, Toolbar, type IDomEditor, type IEditorConfig } from '@wangeditor/editor';
	import { onDestroy, onMount } from 'svelte';

    /**
     * 值
     */
    export let value: string = '';

    /**
     * 编辑器配置
     */
    export let editorConfig: any = undefined;
    /**
     * 工具栏配置
     */
    export let toolbarConfig: any = undefined;

    const defaultEditorConfig: Partial<IEditorConfig> = {
        placeholder: '',
        onChange(editor) {
            value = editor.getHtml();
        }
    }

    const defaultToolbarConfig = {

    }

    let toolbarContainer:HTMLElement;
    let editorContainer:HTMLElement;

    
    let editor: IDomEditor;
    let toolbar: Toolbar;

    onMount(()=>{
        //
        editor = createEditor({
            selector: editorContainer,
            html: value || '<p><br></p>',
            config: Object.assign({}, defaultEditorConfig, editorConfig),
            mode: 'default', // or 'simple'
        })
        // 
        toolbar = createToolbar({
            editor,
            selector: toolbarContainer,
            config:  Object.assign({}, defaultToolbarConfig, toolbarConfig),
            mode: 'default', // or 'simple'
        })

    })

    onDestroy(()=>{
        editor.destroy();
    })

    $: ewClass = `border z-50  ${$$props.class??''}`;
    $: tcClass = `border-b  ${$$props.tcClass??''}`;
    $: ecClass = `border-0 ${$$props.ecClass??''}`;
    
</script>

<div id="editor—wrapper" class={ ewClass }>
    <div id="toolbar-container" bind:this={ toolbarContainer } class={ tcClass }><!-- 工具栏 --></div>
    <div id="editor-container" bind:this={ editorContainer } class={ ecClass }><!-- 编辑器 --></div>
</div>

<style lang="postcss">

</style>