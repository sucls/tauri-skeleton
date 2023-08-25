<script lang="ts">

	import Seacher from "$lib/components/Seacher/Seacher.svelte";

</script>

<div class="flex-auto w-full h-full flex overflow-hidden">
    <div class="flex-none overflow-x-hidden overflow-y-auto bg-surface-50-900-token lg:w-auto border-r border-surface-500/30">
        <!--  search  -->
        <div class="">
            <Seacher querySelection={ (input)=>[] }  inputClass="p-1 border"/>
        </div>
        <!-- SubMenus --> 
        <section class="px-0 py-2">

        </section>
    </div>
    <!--  page content  -->
    <div class="overflow-y-auto flex-1 overflow-x-hidden flex flex-col ">
        <slot/>
    </div>
</div>