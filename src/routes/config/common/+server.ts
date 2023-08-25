
import { json } from '@sveltejs/kit'

export const GET = ( {request} )=>{

    console.log( request )

    return json({
        name: 123
    });
}
