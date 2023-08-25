<script lang="ts">
    import { AppBar } from '@skeletonlabs/skeleton';
    import IonIcon from '$lib/icon/IonIcon.svelte';
</script>

<AppBar {...$$props}>
    <svelte:fragment slot="lead">
        <slot name="lead"/>
    </svelte:fragment>

    <svelte:fragment slot="trail">
        <slot name="trail"/>
    </svelte:fragment>

    <svelte:fragment slot="headline">
        
    </svelte:fragment>
</AppBar>