## 桌面端应用

参考**网易邮箱大师**

### 技术栈

1. Tauri
2. Rust
3. Svelte
4. SvelteKit
5. TailWindCss
6. skeleton

### 前端

1. 安装
```bash
npm create skeleton-app@latest tauri-skeleton
```

### 后台

2. 安装
安装cli
```bash
npm i -D @tauri-apps/cli
```
初始化
```
npm run tauri init
```

### 图标库

选择 ionicons

```html
<script type="module" src="https://unpkg.com/ionicons@7.1.0/dist/ionicons/ionicons.esm.js"></script>
<script nomodule src="https://unpkg.com/ionicons@7.1.0/dist/ionicons/ionicons.js"></script>
```

```html
<ion-icon name='heart' suze='small'></ion-icon>
```

具体可参考：
https://ionic.io/ionicons/usage

### 项目结构

### 问题集