export type List = Array<{ href: string; label: string; keywords: string; badge?: string, icon?: string, expanded?: boolean }>;

export const menuNavLinks: Record<string, Array<{ id?: string, title: string; icon?: string, custom?: boolean, list: List }>> = {
    'email': [
        {
            title: '快捷文件夹',
            list: [
                { href:'/email/all-inboxs', label:'所有收件箱', keywords:'', icon: 'file-tray' },
                { href:'/email/all-unreads', label:'所有未读', keywords:'', icon: 'ellipse'},
                { href:'/email/all-flags', label:'所有红旗', keywords:'', icon: 'flag' },
                { href:'/email/all-drafts', label:'所有草稿箱', keywords:'', icon: 'document' },
                { href:'/email/all-sents', label:'所有已发送', keywords:'', icon: 'paper-plane' },
            ]
        },
        {
            title: '邮箱',
            custom: true,
            icon: '',
            list: [
                { href:'/email/emails', label:'sucl@126.com', keywords:'' }
            ]
        }
	],
    'workspace': [
        {
            title: '全部文件',
            icon: 'cube',
            list: [
                { href:'/workspace/files', label:'文档', keywords:'', icon: 'document' },
                { href:'/workspace/pictures', label:'图片', keywords:'', icon: 'image' },
                { href:'/workspace/musics', label:'音乐', keywords:'', icon:'musical-notes' },
                { href:'/workspace/videos', label:'视频', keywords:'', icon:'videocam' },
                { href:'/workspace/packages', label:'压缩包', keywords:'', icon:'library' },
                { href:'/workspace/others', label:'其他', keywords:'', icon: 'ellipsis-horizontal-circle' },
            ]
        },
        {
            title: '任务列表',
            list: [

            ]
        },
    ],
    'contact': [],
    'calendar': [],
    'seeting': [],
}

export type MenuType = typeof menuNavLinks;
