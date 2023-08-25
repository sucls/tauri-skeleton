## SvekteKit

### Layout

#### 定义

+ +layout.svelte

#### 继承

由于svelte的请求路径与文件路径一致
每一级别的layout都会继承

`除非使用@打破继承关系，+layout@.svelte`

### Page

#### 定义

+ +page.svelte

```svelte
<script>
    export let data;

    function click(){

    }

    $: count = 1;
</script>

<div>

</div>

<style>

</style>

```

### data

```js
<script>
    export let data;
</script>
```

#### config

在+page.svelte中可以定义config
```js
/** @type {import('some-adapter').Config} */
export const config = {
    runtime: 'edge'
};
```
多级别的config可以合并，但不是深度合并

### Load

+ +page.js

示例
```js
export function load( { fetch, params } ){

    return {
        name: 'kit'
    }
}
```

`该方法导出的数据，可以在+page.svelte通过 export let data; 接`

```html
<script>
    export let data;
    const { name } = data;
</script>

```

#### fetch

在load方法中，可以通过fetch方法获取数据，比如：
```js
export async function load( { fetch, params } ){
    const res = await fetch( '/api/user' )
    console.log( await res.json() )
    return {
        name: 'kit'
    }
}
```
以上路径会映射到： /src/routes/api/user/+server.js

+ +server.js
```js
import { json } from '@sveltejs/kit'

export function GET( { request } ){
    return json({

    })
}

export function GET( { request } ){
    return json({
        
    })
}
 ```

#### parent

load方法和layout一样，可以数据继承
```js
/** @type {import('./$types').PageLoad} */
export async function load({ parent }) {
    const { a, b } = await parent();
    return { c: a + b };
}
```


### Route

+ 有包含[]的
  
用作动态参数路由，[]内容是参数名
```js
// /src/routes/account/[id]/+page.svelte

// route: /account/1

export function load({ params }) {
    console.log(params); // {id: 1}
}
```
当然可以支持多个值，比如这样 /a/[...p]/ , url: /a/b/c，其中p则为 b/c

+ 有包含()的

如果目录结构中包含()，不会作为route，比如(app)
/src/routes/(app)/account/+page.svelte
route: /account

其主要作用是分离layout

+ 有包含+x的

### 打包

根据环境选中打包适配器

+ @sveltejs/adapter-cloudflare for Cloudflare Pages
+ @sveltejs/adapter-cloudflare-workers for Cloudflare Workers
+ @sveltejs/adapter-netlify for Netlify
+ @sveltejs/adapter-node for Node servers
+ @sveltejs/adapter-static for static site generation (SSG)
+ @sveltejs/adapter-vercel for Vercel
+ Additional [community-provided adapters](https://sveltesociety.dev/components#adapters)

```
Could not detect a supported production environment. See https://kit.svelte.dev/docs/adapters to learn how to configure your app to run on the platform of your choosing
```

+ Zero Config

```
npm i -D @sveltejs/adapter-static
```