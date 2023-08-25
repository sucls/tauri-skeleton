<script lang="ts">

    import { invoke } from '@tauri-apps/api';
    import { listen, emit } from "@tauri-apps/api/event";
    import { onMount } from "svelte";

    import PageContainer from '$lib/components/PageContainer/PageContainer.svelte';

    let msg1 = '123';
    let msg2 = '';
    let asyncMsg: unknown = '';

    /**
     * 测试与rust通信
     * invoke_handler
     */
    const onClick = async (evt) => {
        msg1 = await invoke('dispatch', {command: 'test', args: '1,2,3'})
    }

    /**
     *
     */
    const onInvokeClick = async (evt) => {
        msg2 = await invoke('plugin:calculate|add', {v1: 5, v2: 7})
    }

    const onSyncClick = async (evt) => {
        let res = await emit('window-message', { msg: 'MESSAGE OF WINDOW' });
        console.log( 'emit' , res )
    }

    onMount(async () => {
        await listen('window-message', async evt =>{
            // 监听事件
            console.log(` 接收事件：【window-message】消息 ${evt.payload}`);
            asyncMsg = evt.payload;
        })
    })

</script>

<PageContainer>
	<div class="space-y-5 w-full">
		<h1 class="h1">Tauri</h1>
        <div>
            <button type="button" class="btn variant-filled-surface" on:click={onClick}>获取后台数据1</button>
            <span>结果：{msg1}</span>
        </div>
    
        <div >
            <button type="button" class="btn variant-filled-surface" on:click={onInvokeClick}>获取后台数据2</button>
            <span>结果：{msg2}</span>
        </div>
    
        <div >
            <button type="button" class="btn variant-filled-surface" on:click={onSyncClick}>页面间取数</button>
            <span>结果：{asyncMsg}</span>
        </div>
    </div>
</PageContainer>


<style lang="postcss">

</style>