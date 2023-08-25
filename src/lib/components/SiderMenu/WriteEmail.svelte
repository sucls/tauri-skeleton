<script lang="ts">

	export let parent: any; // Modal
	
	// Stores
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';

	import Editor from '$lib/components/Editor/Editor.svelte';

	import { onMount } from "svelte";
	import { invoke } from '@tauri-apps/api';
	import type { EmailMessage } from '@/types';

	import List from '$lib/components/List/List.svelte';
	import type { ListItem } from '$lib/components/List/List';


	let email: EmailMessage = {
		id: -1, // 标识新邮件
		source: '409835152@qq.com',
		target: 'su_chunlong@126.com',
		subject: '中午好',
	    html: '你好，每一天 ',
	};

	const popupSetting: PopupSettings = {
		event: 'click',
		target: 'popup-emails',
		placement: "bottom",
		middleware: { offset: { bottom: 0 } }
	}

	let emailItems: ListItem[] = [
		{label:"123@qq.com" },
		{label:"abc@126.com" },
	];

	onMount(()=>{
		
	})

	const sendEmail = (evt: Event)=>{
		console.log( '开发发送邮件...', email );
		invoke('plugin:email|send', { emailJson: JSON.stringify(email) });
	}
</script>


<div class="mx-0 flex justify-center items-start overflow-auto bg-slate-100 p-5">
	<slot/>
	<div class="space-y-5 w-full">
		<div class="flex-1">
			<div class="p-4 w-full text-token space-y-4">
				<div class="input-group rounded-none bg-inherit border-0 border-b">
					<input type="text" name="target" placeholder="收件人" bind:value={ email.target }/>
				</div>
				<div class="input-group rounded-none bg-inherit border-0 border-b">
					<input class="input" name="subject" type="text" placeholder="主题" bind:value={ email.subject }/>
				</div>

				<Editor class="border-primary-500" ecClass="lg:h-[300px]" bind:value={ email.html }>
					
				</Editor>

				<div class="flex justify-between justify-items-center">
					<div class="space-x-2 flex justify-center justify-items-center">
						<button type="button" class="btn btn-sm px-4 bg-primary-500" on:click={ sendEmail }>发送</button>
						<div class="relative">
							<div data-popup="popup-emails" class="card rounded-none w-full p-4 overflow-y-auto">
								<List items = { emailItems }/>
							</div>
							<a href="###" use:popup={ popupSetting } class="hover:bg-emerald-100 p-2">{email.source}</a>
						</div>
					</div>
					<div class="space-x-2 flex flex-row justify-center justify-items-center">
						<label class="flex items-center space-x-2">
							<input class="radio" type="radio" name="radio-direct" value="1" />
							<p>邮件追踪</p>
						</label>
						<label class="flex items-center space-x-2">
							<input class="radio" type="radio" name="radio-direct" value="2" />
							<p>定时发信</p>
						</label>
						<label class="flex items-center space-x-2">
							<input class="radio" type="radio" checked name="radio-direct" value="3" />
							<p>已读回执</p>
						</label>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<style lang="postcss">

</style>