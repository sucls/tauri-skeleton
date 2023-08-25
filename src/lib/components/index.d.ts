/**
 * 选项
 */
export type Item= {
    id: string,
    name?: string,
    label: string,
    icon?: string,
}

/**
 * 可展开的
 */
export interface ExpandedAble{
    expanded: boolean
}

/**
 * 可选择的
 */
export interface SelectAble{
    selected: boolean
}

/**
 * 可以使用的
 */
export interface Useable{
    disabed: boolean
}