export type ListItem = {
    label: string,
    icon?: string,
    avatar?: string,
    description?: string,
    href?: string,
    children?: ListItem[],// for tree
    expanded?: boolean // for tree
}

export type ListType = 'ul'|'ol'|'desc'|'nav'|'tree';