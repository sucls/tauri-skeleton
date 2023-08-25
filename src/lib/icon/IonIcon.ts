export type IonIcon = BasicIocIcon | SvgIonIcon;

/**
 * IocICON支持属性
 */
export type BasicIocIcon = {
    name: string, /*  variants  */
    size?: 'small'|'large',
    color?: string,
    strokeWidth?: string|number,
    platform?: {[key in Platform] : string},
    aria?: {[key:string]: string}
}

/**
 * Svg支持
 */
type SvgIonIcon = {
    src: string
}

type Platform = 'md'|'ios';

const icon1: IonIcon = {
    name:'heart',
    size: 'small',
    color: 'blue',
    strokeWidth: '16',
    platform: {md: 'heart-sharp', ios: 'heart-outline'},
    aria: {hidden: 'true', label: "Favorite"}
}

// const icon2: IonIcon = {
//     src:''
// }

// console.log( icon1 , icon2)