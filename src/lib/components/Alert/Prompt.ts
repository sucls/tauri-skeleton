import { modalStore } from '@skeletonlabs/skeleton';
import type { ModalSettings } from '@skeletonlabs/skeleton';


export const prompt = (title :string, content: string, value: any, submit: (r:any)=>void)=>{
	const modal: ModalSettings = {
		type: 'prompt',
		title: title,
		body: content,
		value: value,
		valueAttr: { type: 'text', minlength: 3, maxlength: 10, required: true },
		response: (result: string) => {
			submit(result)
		}
	};
	modalStore.trigger(modal);
}