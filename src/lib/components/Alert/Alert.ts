import { modalStore } from '@skeletonlabs/skeleton';
import type { ModalSettings } from '@skeletonlabs/skeleton';


export const alert = (title :string, content: string)=>{
	const modal: ModalSettings = {
		type: 'alert',
		title: title,
		body: content
	};
	modalStore.trigger(modal);
}