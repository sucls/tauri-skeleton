import type { QueryResult } from 'tauri-plugin-sql-api';

import { execute, select, selectOne } from '$lib/datasource/Datasource';

import type { FolderType, EmailMessage, Email, EmailFolder, Folder } from '@/types';
import { loadCommonEmailFolders, loadUserEmailFolders } from './UserConfigService';
import type { Optional } from '@/util/Utilities';


const TABLE_EMAIL = "email";

/**
 * 获取邮箱地址列表
 * 
 * @returns 
 */
export const loadEmails = async () : Promise<Email[]> => {
	return await select<Email>(`select id, address, kind, status from ${TABLE_EMAIL} where status = '1'`);
}

/**
 * 获取邮件列表
 * 
 * @param type 
 * @param user 
 * @returns 
 */
export const queryUserEmailsWithType = (type: FolderType, user: string) : EmailMessage[] =>{
    console.info( `根据参数【${type} ${user}】查询邮件列表` )   
    // todo
    return [
        {id: 1, subject: '关于20203年7月1日的会议纪要', text:'这是一段文字', source:'aily@123.com', html: ''},
        {id: 2, subject: '关于中秋节放假通知', text:'这是一段文字', source:'bee@123.com',html: ''},
        {id: 3, subject: '重要公告', text:'这是一段文字', source:'covl@123.com',html: ''},
        {id: 4, subject: '你被X公司抽中大奖，请尽快领取', text:'这是一段文字', source:'dany@123.com',html: ''},
    ];
}

/**
 * 根据id查询邮件内容
 * @param id 
 */
export const queryEmailById = (id: string): EmailMessage =>{
    console.info( `根据id【${id}】查询邮件内容` )   
    // todo
    return {
        id: 1,
        target: 'aily@123.com',
        subject: '周末一起聚会啊！！！',
        source: 'covl@123.com',
        receive_date: new Date(),
	  	html: 'xxxx'
    }
}

export const loadEmailFolders = async (emails?: Email[]): Promise<EmailFolder[]>=>{
  emails = emails || await loadEmails();
    const commonFolders: Folder[] = await loadCommonEmailFolders();
    return mergeEmailAndFolder( emails, commonFolders );
}

const mergeEmailAndFolder = (emails: Email[], commonEmailFolders: Folder[]): EmailFolder[]=> {
    if( !emails.length ){
        return [];
    }
    const emailFolders: EmailFolder[] = [];
  emails.forEach( email =>{
        const addressFolders: Folder[] = loadUserEmailFolders( email );
        emailFolders.push( Object.assign( {}, email, {folders: commonEmailFolders.concat(addressFolders).sort( folderComparer )} ) );
    })
    return emailFolders
}

const folderComparer = (f1: Folder, f2: Folder): number=>{
    return compare(f1, f2, 'order');
}

const compare = (a: any, b: any, prop: string|undefined): number =>{
    if( !a && !b){
        return 0;
    }else if( !a || !b ){
        return !a ? -1:1;
    }else{
        if( prop ){
            return compare(a[prop], b[prop], undefined);
        }else{
            return a - b;
        }
    }
}


/**
 * 保存
 * @param email
 */
export const saveEmail = async (email: Optional<Email, 'id'>):Promise<number> =>{
	if( email.id != undefined ){
		const count: number|undefined = await selectOne<number>(`select count(1) from ${TABLE_EMAIL} where id = $0`, [ email.id ]);
		if( count != undefined && count > 0 ){
			// todo 已存在
			console.error(`${email.id} 已存在！`)
		  return 0;
		}
	}else{
	  const result: { max: number }|undefined = await selectOne<{max: number}>(`select max(id) as max from ${TABLE_EMAIL}`);
	  email.id = (result?result.max:0) + 1;
	}
	const result: QueryResult = await execute(`insert into ${TABLE_EMAIL}(id, address, password, status, kind) values($1,$2,$3,$4,$5)`,[ email.id, email.address, email.password, email.status, email.kind ]);
	return result.rowsAffected;
}
