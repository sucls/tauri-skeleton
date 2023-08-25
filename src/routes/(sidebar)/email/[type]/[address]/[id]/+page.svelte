<script lang="ts">
   import type { PageData } from './$types';
	import { AppBar } from "@skeletonlabs/skeleton";

   import PageContainer from "$lib/components/PageContainer/PageContainer.svelte";

   export let data: PageData;

   let { email } = data;

</script>

<AppBar>
   
</AppBar>

<!-- 邮件内容展示  -->
<PageContainer>
	<h3 class="h3" data-toc-ignore>Test Scrolling Here</h3>
   <span> {email.subject} </span>
   <p>
      {@html email.html}
   </p>
</PageContainer>
			