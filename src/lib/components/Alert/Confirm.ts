import { modalStore } from '@skeletonlabs/skeleton';
import type { ModalSettings } from '@skeletonlabs/skeleton';


export const Confirm = (title :string, content: string, submit: (r:boolean)=>void)=>{
	const modal: ModalSettings = {
		type: 'confirm',
		title: title,
		body: content,
		response: (result: boolean) => {
			submit( result )
		}
	};
	modalStore.trigger(modal);
}