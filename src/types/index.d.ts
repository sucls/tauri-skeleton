export enum EmailTypeCategory {
    /**
     * 收件箱
     */
    INBOX='收件箱',
    /**
     * 未读邮件
     */
    UNREAD='未读邮件',
    /**
     * 所有红旗
     */
    FLAG = '红旗邮件',
    /**
     * 草稿箱
     */
    DRAFT = '草稿箱',
    /**
     * 所有已发送
     */
    SENT = '已发送',
    /**
     * 
     */
    JUNK = '垃圾有奖',
    /**
     * 
     */
    DELETED = '已发送'
}

/**
 * 邮箱子菜单（快捷文件夹）
 */
export type FolderType = 'all-inboxs'|'all-unreads'|'all-flags'|'all-drafts'|'all-sents';

/**
 * 页面请求参数
 */
export type PathParam = {
    type: FolderType,
    address: string,
}

/**
 * 邮箱
 */
export type Email = {
    id?: number,
    address: string,
    password: string,
    kind?: string, // 126 qq ...
    status?: string,
}

/**
 * 邮件
 */
export type EmailMessage = {
    id: number, // ID
    subject: string, // 邮件标题
    source: string, //
    target?: string, //
    cc?: string, // 抄送
    bcc?: string, // 密送
    reply_to?: string,
    category?: string, // 邮件分类
    folder?: string, // 邮件文件夹
    receive_date?: Date,
    send_date?: Date,
    text?: string, //
    html: string,
}

/**
 * 附件类型
 */
export type Attachment = {
    id: number,
    message_id: number,
    name: string,
    kind: string, // text html file
    path: string,
    data: string
}

/**
 * 可展开
 */
export interface ExpandedAble{
    expanded?: boolean;
}

/**
 * 邮箱文件夹
 */
export type EmailFolder = Email & {folders: Folder[]}

/**
 * 邮箱文件夹
 */
export type Folder = {
    id: number,
    email_id?: number,
    name: string,
    icon?: string,
    sort?: number
}