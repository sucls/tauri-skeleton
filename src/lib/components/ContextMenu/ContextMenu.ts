import type { PopupSettings } from "@skeletonlabs/skeleton";
import type { Item } from "..";

/**
 * 
 */
export type MenuItem = {
    group?: string,
    hotKye?: string,
    children?: MenuItem[],
} & Item;

/**
 *  位置
 */
export type Position = {
    x: number,
    y: number,
    offset?: number,
}

/**
 * 
 */
export type ContextMenuProps = {
    menuItems: MenuItem[],
    position: Position,
    visiable: boolean
}

/**
 * use:popup
 * data-poput
 */
export const contextMenuPopup: PopupSettings = {
    event: 'click',
    target: 'contextMenuPopup',
    placement: 'right',
    middleware:{
        // offset: 15
    }
};