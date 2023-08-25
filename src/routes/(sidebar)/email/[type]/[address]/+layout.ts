import type { Load } from "@sveltejs/kit";
import type { EmailMessage, PathParam } from "@/types";

import { queryUserEmailsWithType } from '@/service/EmailService';

/**
 * 
 * @param param0 
 * @returns 
 */
export const load: Load = async ({ params, route })=>{

    const { type, address } = { ... params } as PathParam;

    const emails: EmailMessage[] = await queryUserEmailsWithType( type, address );

    return {
        emails
    }
}