# Winterfox

一个基于 Tauri 的桌面应用程序。

## GitHub Actions 自动化构建

项目配置了以下 GitHub Actions 工作流：

### 1. CI 工作流 (`ci.yml`)

- **触发条件**: 推送到 `main` 或 `develop` 分支，或创建 Pull Request
- **功能**:
  - 运行代码检查和测试
  - 在多个平台（Ubuntu、Windows、macOS）上构建应用
  - 确保代码质量

### 2. 发布工作流 (`release.yml`)

- **触发条件**: 推送以 `v` 开头的标签（如 `v1.0.0`、`v2.1.0-beta.1`）
- **功能**:
  - 自动创建 GitHub Release
  - 在多个平台构建 Tauri 应用
  - 上传构建产物到 Release 页面
  - 支持跨平台构建（macOS ARM64/x64、Windows、Linux）

## 使用方法

### 本地开发

```bash
# 安装依赖
pnpm install

# 启动开发服务器
pnpm dev

# 构建应用
pnpm build

# 运行测试
pnpm test

# 代码检查
pnpm lint
```

### 发布新版本

1. 更新版本号：

   ```bash
   # 更新 package.json 版本
   npm version patch  # 或 minor, major
   ```

2. 创建并推送标签：

   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

3. GitHub Actions 会自动：
   - 检测到标签推送
   - 创建 GitHub Release
   - 在多平台构建 Tauri 应用
   - 上传构建产物到 Release 页面

### 支持的平台

- **Windows**: `.exe` 安装程序
- **macOS**: `.dmg` 和 `.app` 文件（ARM64 和 x64）
- **Linux**: `.AppImage` 和 `.deb` 包
- **Android**: `.apk` 文件（ARM64、ARMv7、x86、x86_64）

### Android 构建

项目包含专门的 Android 构建工作流，支持多架构 APK 生成。详细说明请查看 [docs/android-build.md](docs/android-build.md)。

## CLI Release 工具

Winterfox 包含一个强大的 CLI 工具，可以在 GitHub Actions 工作流初始化环境后接管构建和发布流程。

### 主要功能

- **多平台构建**：支持 Windows、macOS、Linux、Android
- **自动发布**：自动创建 GitHub Release 并上传构建产物
- **灵活配置**：通过配置文件控制构建行为
- **增量操作**：支持只构建或只发布

### 使用方法

```bash
# 本地测试构建和发布
npx winfox release --build-only

# 在 GitHub Actions 中使用
npx winfox release --tag v1.0.0 --token $GITHUB_TOKEN
```

详细文档请查看 [docs/cli-release.md](docs/cli-release.md)。

## 环境要求

- Node.js 20+
- Rust 稳定版
- pnpm 8+
- 平台特定的构建工具（已在 CI 中自动安装）

## 注意事项

1. 确保 GitHub 仓库中启用了 Actions
2. 首次运行可能需要配置 GitHub Token 权限
3. macOS 构建需要 GitHub Actions 的 macOS 运行器
4. Windows 构建需要 WiX Toolset（已在 CI 中自动安装）
5. CLI release 工具需要构建 @winterfox/cli 包

## 开发环境配置

### 推荐 IDE 设置

[VS Code](https://code.visualstudio.com/) + [Vue (Official)](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (禁用 Vetur)。

### TypeScript 对 `.vue` 导入的支持

TypeScript 默认无法处理 `.vue` 导入的类型信息，因此我们使用 `vue-tsc` 替代 `tsc` 进行类型检查。在编辑器中，需要 [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) 来让 TypeScript 语言服务识别 `.vue` 类型。

## Customize configuration

See [Vite Configuration Reference](https://vite.dev/config/).

## Project Setup

```sh
pnpm install
```

### Compile and Hot-Reload for Development

```sh
pnpm dev
```

### Type-Check, Compile and Minify for Production

```sh
pnpm build
```

### Run Unit Tests with [Vitest](https://vitest.dev/)

```sh
pnpm test:unit
```

### Lint with [ESLint](https://eslint.org/)

```sh
pnpm lint
```
