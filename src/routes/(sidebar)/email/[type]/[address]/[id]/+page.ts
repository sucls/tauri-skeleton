import type { Load } from "@sveltejs/kit";

import { queryEmailById } from '@/service/EmailService';
import type { EmailMessage } from "@/types";

/**
 * 
 * @param param0 
 * @returns 
 */
export const load: Load = ( {params} ) =>{
    const id: string = params.id as string;
    const email: EmailMessage = queryEmailById(id);
    return {
        email
    }
}