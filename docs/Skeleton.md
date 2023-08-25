## Skeleton

### 安装

+ 一步安装

```bash
npm create skeleton-app@latest <你的项目名称>
```

+ 手动安装

 1. svelte
```bash
npm create svelte@latest my-skeleton-app
cd my-skeleton-app
npm install
```

2. skeleton
```bash
npm i @skeletonlabs/skeleton --save-dev
```

3. tailwindcss
```bash
npx svelte-add@latest tailwindcss
npm install

```
修改配置 /src/app.postcss，替换内容为：
```js
/** @type {import('tailwindcss').Config} */
module.exports = {
	// 1. Apply the dark mode class setting:
	darkMode: 'class',
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		// 2. Append the path for the Skeleton NPM package and files:
		require('path').join(require.resolve(
			'@skeletonlabs/skeleton'),
			'../**/*.{html,js,svelte,ts}'
		)
	],
	theme: {
		extend: {},
	},
	plugins: [
		// 3. Append the Skeleton plugin to the end of this list
		...require('@skeletonlabs/skeleton/tailwind/skeleton.cjs')()
	]
}
```
4. 主题
   
修改/src/app.html
```html
<html class="dark">
```

5. Stylesheets
   src/routes/+layout.svelte
```js
// Your selected Skeleton theme:
import '@skeletonlabs/skeleton/themes/theme-skeleton.css';

// This contains the bulk of Skeletons required styles:
import '@skeletonlabs/skeleton/styles/skeleton.css';

// Finally, your application's global stylesheet (sometimes labeled 'app.css')
import '../app.postcss';
						
```

---
---

基于TailWindCss实现的组件

### class

+ lg: 
  
  以lg开头的响应式样式
  比如 lg:grid、lg:hidden

```css
    @media(min-width=1024px)
    .lg\:grid{
        display: grid;
    }

    @media(width=1024px)
    .lg\:grid{
        display: hidden;
    }
```

主要基于media实现不同尺寸时的样式是否有效

+ token