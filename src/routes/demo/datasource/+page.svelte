<script lang='ts'>

  import { select, execute } from '$lib/datasource/Datasource';
  import PageContainer from '$lib/components/PageContainer/PageContainer.svelte';

  let sql = 'insert into test(id, title, status) values(1, \'test\', \'01\')';

  let result: any;

  const executeSql = async () => {
	if (sql.trim().startsWith('select')) {
	  result = await select<any[]>(sql);
	} else {
	  result = await execute(sql);
	}
	console.log(`执行sql: ${sql} 结果: ${result}`);
  };

</script>

<PageContainer>
	<div class='space-y-5 w-full flex-1'>
		<h1 class='h1'>Datasource</h1>
		<div class='flex-1'>
			<div class='card p-4 w-full text-token space-y-4'>
				<label class='label'>
					<span>SQL</span>
					<!-- cspell:disable-next-line -->
					<textarea class='textarea' name='sql' rows='4' bind:value={sql} />
				</label>
				<label class='label'>
					<span>结果</span>
					<!-- cspell:disable-next-line -->
					<textarea class='textarea' name='sql' rows='4' value={ result?JSON.stringify(result):''} disabled />
				</label>

				<div class='flex justify-center justify-items-center'>
					<button type='button' class='btn bg-primary-500' on:click={ executeSql }>执行</button>
				</div>
			</div>
		</div>
	</div>
</PageContainer>

<style lang='postcss'>

</style>