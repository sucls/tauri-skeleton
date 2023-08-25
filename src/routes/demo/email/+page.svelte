<script lang="ts">

	import PageContainer from "$lib/components/PageContainer/PageContainer.svelte";

	const sendEmail = (evt: Event)=>{
		console.log( '开发发送邮件...' )
	}
</script>
<!-- YOU CAN DELETE EVERYTHING IN THIS PAGE -->

<PageContainer>
	<div class="space-y-5 w-full">
		<h1 class="h1">Email</h1>
		<div class="flex-1">
			<div class="card p-4 w-full text-token space-y-4">
				<div class="input-group input-group-divider grid-cols-[auto_1fr_auto]">
					<div class="input-group-shim">收件人</div>
					<input type="text" name="to" placeholder="收件人邮箱" />
					<button class="variant-filled-secondary">选择</button>
				</div>
				<label class="label">
					<span>邮件主题</span>
					<input class="input" name="submit" type="text" placeholder="邮件主题" />
				</label>
				<label class="label">
					<span>发件人</span>
					<input class="input" name="from" type="text" placeholder="发件人" value="409835152@qq.com"/>
				</label>
				<label class="label">
					<span>授权码</span>
					<input class="input input-warning" name="token" type="text" placeholder="授权码" value=""/>
				</label>

				<label class="label">
					<span>服务器类型</span>
					<select name="serverType" class="select">
						<option value="2">SMTP</option>
						<option value="1">POP3</option>
					</select>
				</label>
				<label class="label">
					<span>服务器地址</span>
					<input class="input" name="server" type="text" placeholder="服务器地址" value="smtp.qq.com"/>
				</label>
				<label class="label">
					<span>服务器端口</span>
					<!--  587  -->
					<input class="input" name="port" type="text" placeholder="端口" value="465"/>
				</label>


				<label class="label">
					<span>邮件内容</span>
					<!-- cspell:disable-next-line -->
					<textarea class="textarea" name="html" rows="4" placeholder="请输入邮件内容..." />
				</label>

				<div class="flex justify-center justify-items-center">
					<button type="button" class="btn bg-primary-500" on:click={ sendEmail }>发送</button>
				</div>
			</div>
		</div>
	</div>
</PageContainer>

<style>

</style>