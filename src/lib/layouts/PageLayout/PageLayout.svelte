<script lang="ts">
    import { page } from '$app/stores';

</script>

<!--  左右布局  -->
<div class="flex-auto w-full h-full flex overflow-hidden">
    <!-- 左侧导航 -->
    <div class="flex-none overflow-x-hidden overflow-y-auto bg-surface-50-900-token lg:w-auto border-r border-surface-500/30">

        <section class="px-0 py-2">
            <slot name="head"></slot>
        </section>
        <!--  -->
        <slot/>

        <section class="px-0 py-2">
            <slot name="tail"></slot>
        </section>
    </div>
    <!--  右侧页面  -->
    <div class="overflow-y-auto flex-1 overflow-x-hidden flex flex-col ">
        <slot name="pager"></slot>
    </div>
</div>