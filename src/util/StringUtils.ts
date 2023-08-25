/**
 * 
 * @param str 
 * @returns 
 */
const findFirstUp = (str: string): string =>{
    if( !str ){
        return '';
    }
    return str.substring(0,1).toUpperCase();
}

/**
 * 
 * @param str 
 * @returns 
 */
const upFirst = (str: string): string =>{
    if( !str ){
        return '';
    }
    return str.substring(0,1).toUpperCase() + str.substring(1);
}

/**
 * 
 * @param str 
 * @returns 
 */
const findAvatarInitials = (...str: Array<string|undefined>): string =>{
    if( !str || !str.length){
        return '';
    }
    for(const v of str){
        if(v){
            if( v.length > 2 ){
                return v.substring(0,2).toLowerCase();
            }
            return v.toLowerCase();
        }
    }
    return '';
}

export {
    findFirstUp,
    upFirst,
    findAvatarInitials,
}