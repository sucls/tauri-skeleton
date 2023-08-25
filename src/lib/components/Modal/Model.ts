import { modalStore } from '@skeletonlabs/skeleton';
import type { ModalSettings, ModalComponent } from '@skeletonlabs/skeleton';


/**
 * 
 * @param title 
 * @param modal 
 * @param settings 
 */
export const showModel = ( title: string ,modal: ModalComponent , settings ?:ModalSettings)=>{
    const modalSettings: ModalSettings = {
        type: 'component',
        component: modal,
        title: title,
        response: (result: any) => {
            console.log( `submit: ${result}` );
        }
    };
    Object.assign( modalSettings, settings ); //
    modalStore.trigger(modalSettings);
}