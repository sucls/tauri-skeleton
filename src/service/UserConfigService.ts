import type { Email, Folder } from "@/types"

/**
 *  加载用户邮箱文件夹配置
 * 获取用户配置 比如：其他文件夹、垃圾邮件、广告邮件 订阅邮件 ...
 */
export const loadUserEmailFolders = (email: Email): Folder[] =>{
    const emailId = email.id;
    console.log( `获取邮箱【${emailId}】自定义文件夹` );

    return [
        {id: 1, name: '其他文件夹', sort: 10},
        {id: 2, name: '垃圾邮件', sort: 11},
        {id: 3, name: '红旗邮件', sort: 12},
        {id: 4, name: '广告邮件', sort: 13},
        {id: 5, name: '订阅邮件', sort: 14},
    ];
}

/**
 *  
 *  获取公共配置 比如：收件箱、未读邮件、红旗邮件、草稿箱、已发送 ... 已删除
 * 
 * 获取邮箱公共文件夹
 * @returns 
 */
export const loadCommonEmailFolders = (): Folder[] =>{
    return [
        {id: 1, name: '收件箱', sort: 1},
        {id: 2, name: '未读邮件', sort: 2},
        {id: 3, name: '红旗邮件', sort: 3},
        {id: 4, name: '草稿箱', sort: 4},
        {id: 5, name: '已发送', sort: 5},
        {id: 99, name: '已删除', sort: 99},
    ];
}