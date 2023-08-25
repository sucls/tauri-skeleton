<script lang="ts">
	export let parent: any; // Modal

	// Stores
	import { modalStore } from '@skeletonlabs/skeleton';

	import { saveEmail } from '@/service/EmailService';

	let error = ''; // 错误信息

	export let formData: any = {
		address: '',
		password: ''
	};

	const onSubmit = async (evt: MouseEvent) => {
		console.log('login modal submit', formData);
		let result = await saveEmail({ address: formData.address, password: formData.password, status: '1'});
		if (result > 0) {
			$modalStore[0] && $modalStore[0].response && $modalStore[0].response( result );
			modalStore.close();
		}
	};

	// Base Classes
	const cBase = 'card p-4 w-modal-slim shadow-xl space-y-4';
	const cHeader = 'text-2xl font-bold';
	const cForm = ' border-surface-500 p-4 space-y-4 rounded-container-token';
	const cModalImage = 'mx-auto p-5';
</script>

{#if $modalStore[0]}
	<div class="modal-example-form {cBase}">
		<!-- <header class={cHeader}>{$modalStore[0].title ?? ''}</header> -->
		<header class={cHeader}>
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<ion-icon class='float-right text-tertiary-300 hover:cursor-pointer hover:text-tertiary-400' name="close" 
			on:click={()=>modalStore.close()}/> 
		</header>
		<article>{$modalStore[0].body ?? ''}</article>
		<form class="modal-form {cForm}">
			<!-- Image -->
			{#if $modalStore[0]?.image && typeof $modalStore[0]?.image === 'string'}
				<img class="modal-image {cModalImage}" src={$modalStore[0]?.image} alt="Modal" />
			{/if}
			{#if error}
			<div class="w-full text-center p-0 m-0">
				<span class="text-rose-600 text-sm">{error}</span>	
			</div>
			{/if}
			<label class="label">
				<!-- <span> 邮箱 </span> -->
				<input
					class="input focus:border-sky-500"
					type="email"
					bind:value={formData.address}
					placeholder="163/QQ/Gmail/企业邮箱等"
				/>
			</label>
			<label class="label">
				<!-- <span> 密码 </span> -->
				<input
					class="input focus:border-sky-500"
					type="password"
					bind:value={formData.password}
					placeholder="密码"
				/>
			</label>
		</form>
		<footer class="modal-footer {parent.regionFooter}">
			<!-- <button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{parent.buttonTextCancel}</button> -->
			<button class="btn w-full mx-4 {parent.buttonPositive}" on:click={onSubmit}>登录</button>
		</footer>
	</div>
{:else}
	<div>
		<slot />
	</div>
{/if}
