export type TreeItem = {
    id?: string,
    text: string,
    icon?: string,
    expanded?: boolean,
    selected?: boolean,
    children?: TreeItem[],
}

export type Tree = {
    title: string
}

