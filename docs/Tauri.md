##

## 安装

+ 下载依赖
```bash
npm install --save-dev @tauri-apps/cli
```

+ 修改package.json
```json
{
  "scripts": {
    "tauri": "tauri"
  }
}
```

+ 初始化项目
```bash
npm run tauri init
```

+ 启动项目
```bash
npm run tauri dev
```

### 通信

+ 安装依赖
```bash
npm install @tauri-apps/api
```

+ 前端到Rust

基于API
```js

    import { invoke } from '@tauri-apps/api';

    let result = invoke('', message)

```

基于插件：
```rust
// ...
    tauri::Builder::default()
        .plugin(plugins::greet_plugin::init());
// ...
```
```js
    import { invoke } from '@tauri-apps/api'

    let result = await invoke('plugin:<plugin_name>|<method_name>',{ args })
```

基于handler:
```rust
// ...
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![dispatch]);
// ...
```

```js
    import { invoke } from '@tauri-apps/api'

    let result = await invoke('<handle_name>', { args })
```

+ Rust到前端


+ 页面间
  
基于事件监听
```js
// 应用监听事件
await listen('window-message', async event => {

})

// 事件分发
await emit('window-message', { payload });
// 原理
invoke('tauri',{__tauriModule: 'Event',windowLabel:'main', message: {cmd: 'emit',event:'window-message', payload: ""}});
```

### Sqlite

https://github.com/tauri-apps/tauri-plugin-sql