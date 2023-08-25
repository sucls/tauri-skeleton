<script lang="ts">
    type Size = 'small'|'large'|'middle';

    const OUTLINE: string = 'outline'; // 默认
    const BASIC: string = ''; // 选中

    /**
     * 名称；可以通过后缀区分效果
     */
    export let name: string;
    /**
     * 
     */
    export let title: string = '';
    /**
     * 图标大小
     * small 14
     * middle
     * large 32
     */
    export let size: Size|number = 16;
    /**
     * 颜色
     */
    export let color: string|'' = '';
    /**
     * 填充程度
     */
    export let strokeWidth: number = 12;
    /**
     * 多平台支持
     * eg:
     *  ios="heart-outline"
     *  md="heart-sharp"
     */
    export let platform: {[key in 'md'|'ios'] : string} | undefined = undefined;
    /**
     * 辅助元素
     * eg:
     *  aria-hidden="true"
     *  aria-label="Favorite"
     */
    export let aria: {[key:string]: string} = {};

    /**
     * 选中状态
     */
    export let active: boolean = false;
    /**
     * 悬停状态
     */
    export let hover: boolean = false;

    /**
     * 
     */
    let rootstyle: string = '';

    export { rootstyle as style };

    // 用来控制变量
    let styles:string[] = [`--ionicon-stroke-width: ${strokeWidth??0}px`];
    // size支持定义的名称或者px值
    let sizeValue: unknown;

    /**
     * 
     * @param active
     */
    const toggleOutline = (name: string, active: boolean)=>{
        if( name.startsWith('logo') ){ // logo 没有变种样式
            return name;
        }
        if( active){
            if( name.endsWith(OUTLINE) ){
                name = name.replace('-'+OUTLINE, BASIC)
            }
        }else if(!name.endsWith(OUTLINE)){
            name = name + '-' + OUTLINE
        }
        return name;
    }

    /**
     * 
     * @param val
     */
    const calculateSize = (val: any): unknown =>{
        if( !isNaN(val) ){
            styles.push(`--ionicon-font-size: ${val}px`);
            styles = styles;
            return;
        }
        return val;
    }


    const toggleHover = (v: boolean)=>{
        // console.log( 'hover' )
        hover = v;
    }

    // $: classesBase = `ion-icon-ele ${$$props.class??''}`;

    $: sizeValue = calculateSize(size);
        
    $: name = toggleOutline(name, active);

    $: {
        if( color ){
            styles.push(`--ionicon-color: ${color}`);
        }
        if( strokeWidth ){
            styles.push(`--ionicon-stroke-width: ${strokeWidth}px`);
        }
        if( typeof size == 'number' ){
            styles.push(`--ionicon-font-size: ${size}px`);
        }
        styles = styles; //
    }

    $: style = styles && styles.join(';');
    
</script>

<!--  inline-grid  -->
<i class="ion-icon {$$props.class??''}" 
    {title} 
    style = {rootstyle}
    on:mouseenter = { ()=>toggleHover(true) }
    on:mouseleave = { ()=>toggleHover(false) }
>
    <ion-icon
        style = {style}
        class = "ion-icon-ele"
        {name}
        size = {sizeValue}
        {... aria}
        {... platform}
        on:hover
        on:click
        on:mouseenter
        on:mouseleave
    >
    </ion-icon>
    <span> <slot/> </span>
</i>

<style>
    ion-icon{
        margin: 0;
        padding: 0;
        visibility: initial; 
        /* --ionicon-stroke-width: 16, */
        color: var(--ionicon-color);
        font-size: var(--ionicon-font-size);
    }
    ion-icon:hover{
        cursor: pointer;
    }
    :global(.ion-icon){
        line-height: 0;
        display:inline-block;
    }
</style>