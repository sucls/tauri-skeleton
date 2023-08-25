<script lang="ts">
    import { page } from '$app/stores';
	import { goto } from "$app/navigation";


    // 约定名称： ALL 由于布局原因，所有路径必须是 /email/<type>/<address>，因此将点击分类时， ALL作为address
    goto($page.url.pathname + '/' + 'ALL');
</script>