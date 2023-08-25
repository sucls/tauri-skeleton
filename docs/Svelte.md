## Svelte

### 组件

+ 定义
```js

<script>

</script>

<div>

</div>

<style>

</style>
```

+ 适用


+ 组件通信

 - 父->子
基于Context的数据共享
```js
// 父组件中
<Parent/>

import { setContext } from 'svelte';

setContext('sheard', { ... })

// 子组件中
<Child/>

import { getContext } from 'svelte';

const shared = getContext('shared');

```

 - 子->父

基于dispatcher的事件分发
```js

<Comp on:handle = {( {detail} )=>{ ... }}>
</Comp>

// 在组件Comp内部，可以通过dispatch分发事件

	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

    dispatch('handle', { ... })

```

 - bind
数据双向流

### 属性

定义属性的方式很简单：
```svelte
// Account
<script>
    export let name;
</script>
<div>
    ...
</div>
```

bind:<prop>
控住数据流向，数据通常从父级流到子级。 bind: 指令允许从子对象流向父对象

bind:this

### Store

状态管理工具，Svelte中通过Store实现数据状态管理，有三种
writable
readable
derived

+ store的定义

```js
import { writable } from 'svelte/store';

const count = writable(0);
```

+ store的使用


### Context


### 生命周期


### Event

