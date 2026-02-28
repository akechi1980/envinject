# envinject

`envinject` 是一个跨平台的单文件工具，用于按“项目”管理环境变量，并在运行项目命令时临时注入这些变量。

默认作为 CLI 使用；当执行 `envinject gui` 时，会打开本地图形界面用于编辑配置。

---

## 设计目标（MVP）

- 单个可执行文件分发（不拆分 CLI/GUI 为两个程序）
- 默认命令行模式
- `gui` 子命令打开配置编辑器
- `run <project> -- <command...>` 注入环境变量并执行命令
- 本地配置文件优先，先保证简单稳定

---

## 功能概览

- 按项目存储环境变量（`project -> key/value`）
- 运行命令时自动注入对应项目环境变量
- GUI 支持：
  - 新建项目
  - 选择项目
  - 编辑键值对
  - 删除键
  - 保存配置
- CLI 支持：
  - `run <project> -- <command...>`
  - `gui`
  - `list`
  - `show <project>`

---

## 技术栈

- Rust 2021
- `clap`：CLI 参数解析
- `eframe/egui`：轻量 GUI
- `serde + serde_json`：配置序列化
- `directories`：跨平台配置目录定位
- `anyhow`：错误处理

---

## 项目结构

```text
envinject/
├── Cargo.toml
└── src/
    ├── main.rs    # 程序入口与子命令分发
    ├── config.rs  # 配置读写（本地 JSON）
    ├── exec.rs    # 命令执行与环境注入
    └── gui.rs     # GUI 配置编辑器
```

---

## 快速开始

### 1) 构建

开发构建：

```bash
cargo build
```

发布构建（推荐）：

```bash
cargo build --release
```

可执行文件位于：

- Linux/macOS: `target/release/envinject`
- Windows: `target\release\envinject.exe`

### 2) 打开 GUI 配置项目变量

```bash
envinject gui
```

如果你还未将可执行文件加入 `PATH`，可直接运行：

```bash
./target/release/envinject gui
```

在 GUI 里完成：

1. 新建项目（如 `myproj`）
2. 添加键值（如 `API_URL`、`DB_HOST`）
3. 点击“保存配置”

### 3) 按项目注入变量并运行命令

Node 示例：

```bash
envinject run myproj -- npm run dev
```

Python 示例：

```bash
envinject run myproj -- python app.py
```

Rust 示例：

```bash
envinject run myproj -- cargo run
```

---

## GitHub Release 自动构建

仓库内已配置 GitHub Actions，在**推送版本标签**时会自动编译多平台二进制并发布到 GitHub Release。

### 触发发布

```bash
git tag v0.1.0
git push origin v0.1.0
```

推送 `v*` 标签后，Actions 会构建并生成 Release，附带以下资产：

| 平台 | 文件名 |
|------|--------|
| Linux (x86_64) | `envinject-linux-x86_64` |
| Windows (x86_64) | `envinject-windows-x86_64.exe` |
| macOS (Apple Silicon) | `envinject-macos-aarch64` |
| macOS (Intel) | `envinject-macos-x86_64` |

用户可在仓库的 **Releases** 页面下载对应平台的单文件可执行程序，无需本地安装 Rust。

---

## Windows / Ubuntu 推荐安装与部署

下面给出“本地单机使用”场景下最实用、最稳妥的安装方式。`envinject` 是单二进制程序，核心思路是：编译出一个文件，放到固定目录并加入 `PATH`。

### Windows（PowerShell）

#### 方式 A：从源码构建（推荐给开发者）

1) 安装 Rust（rustup）  
2) 在项目目录构建：

```powershell
cargo build --release
```

3) 建议安装到用户目录（无需管理员）：

```powershell
New-Item -ItemType Directory -Force "$env:LOCALAPPDATA\Programs\envinject" | Out-Null
Copy-Item ".\target\release\envinject.exe" "$env:LOCALAPPDATA\Programs\envinject\envinject.exe" -Force
```

4) 将目录加入用户 PATH（首次执行）：

```powershell
[Environment]::SetEnvironmentVariable(
  "Path",
  $env:Path + ";$env:LOCALAPPDATA\Programs\envinject",
  "User"
)
```

5) 重新打开终端后验证：

```powershell
envinject list
envinject gui
```

#### 方式 B：从 GitHub Release 下载或直接分发单文件

- 在仓库 **Releases** 页下载 `envinject-windows-x86_64.exe`，重命名为 `envinject.exe` 后放到固定目录（如 `%LOCALAPPDATA%\Programs\envinject\`），并将该目录加入 PATH。
- 或由维护者直接提供 `envinject.exe`，用户同上配置 PATH 后即可运行 `envinject gui` / `envinject run ...`。

### Ubuntu（bash）

#### 方式 A：从源码构建（推荐）

1) 安装 Rust：

```bash
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
```

2) 在项目目录构建：

```bash
./build.sh release
# 或 cargo build --release
```

3) 安装到本机命令目录：

```bash
sudo install -Dm755 ./target/release/envinject /usr/local/bin/envinject
```

4) 验证：

```bash
envinject list
envinject gui
```

#### 方式 B：从 GitHub Release 下载（无需 Rust）

在仓库 **Releases** 页下载 `envinject-linux-x86_64`，然后：

```bash
mkdir -p "$HOME/.local/bin"
install -m755 ~/Downloads/envinject-linux-x86_64 "$HOME/.local/bin/envinject"
echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
source "$HOME/.bashrc"
```

#### 方式 C：仅当前用户安装（无 sudo，从源码构建）

```bash
mkdir -p "$HOME/.local/bin"
install -m755 ./target/release/envinject "$HOME/.local/bin/envinject"
echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
source "$HOME/.bashrc"
```

### 部署建议（Windows / Ubuntu 通用）

- 推荐固定安装路径，升级时只替换二进制文件
- 升级后先执行 `envinject list` 验证配置读取正常
- 配置文件位于用户目录，**升级二进制不会清空配置**
- 团队分发建议附带版本号，例如 `envinject v0.1.0`

---

## CLI 详细说明

### `envinject run <project> -- <command...>`

读取指定项目的环境变量，并注入到目标子进程。

语法：

```bash
envinject run <project> -- <command> [args...]
```

示例：

```bash
envinject run webapp -- npm run dev
envinject run backend -- python app.py
```

说明：

- `--` 之后的内容会作为要执行的命令及参数原样传递
- 子进程继承当前终端标准输入输出
- 退出码与子进程保持一致

### `envinject gui`

启动本地图形配置编辑器，仅用于编辑配置，不做终端托管或日志面板。

### `envinject list`

列出当前所有已保存项目。

### `envinject show <project>`

显示指定项目的所有环境变量（`KEY=VALUE`）。

---

## 配置文件说明

`envinject` 使用本地 JSON 文件存储配置，自动放在系统标准配置目录中。

### 默认路径规则

由 `directories::ProjectDirs::from("dev", "envinject", "envinject")` 解析得到配置目录，再写入 `config.json`。

常见位置通常类似：

- Linux: `~/.config/envinject/config.json`（或等价 XDG 路径）
- macOS: `~/Library/Application Support/dev.envinject.envinject/config.json`（实际路径由系统环境决定）
- Windows: `%APPDATA%\dev\envinject\envinject\config\config.json`（实际路径以系统解析为准）

> 实际路径可通过 GUI 保存后提示信息确认。

### 文件结构示例

```json
{
  "projects": {
    "myproj": {
      "env": {
        "API_URL": "http://localhost:3000",
        "TOKEN": "abc123"
      }
    },
    "another": {
      "env": {
        "MODE": "dev"
      }
    }
  }
}
```

---

## GUI 使用说明（当前版本）

左侧面板：

- 项目列表
- 输入项目名并新建
- 删除当前选中项目

右侧面板：

- 当前项目详情
- 新增/更新键值
- 直接编辑已有值
- 删除某个键

顶部：

- “保存配置”按钮
- 状态提示（保存成功/失败等）

---

## 常见使用场景

### 场景 1：同一台机器多项目隔离配置

- `frontend`、`backend`、`tools` 各自维护变量
- 使用 `run` 时按项目注入，避免串配置

### 场景 2：不在项目目录存真实 `.env`

- 代码统一走 `std::env` / `process.env` / `os.environ`
- 真实值放在 `envinject` 本地配置中管理

### 场景 3：一键切换环境

- `myproj-dev`、`myproj-staging` 分别作为两个项目
- 通过不同 project 名快速切换

---

## 注意事项

- 当前版本不区分普通变量与秘密变量（统一键值编辑）
- 当前版本未引入加密或系统密钥链，适合本地单机使用
- `show` 会明文打印值，请在安全环境使用

---

## 跨平台说明

项目设计上兼容 Windows / Linux / macOS：

- 配置路径由 `directories` 统一处理
- 命令执行使用 Rust 标准库 `std::process::Command`
- GUI 采用 `eframe/egui`，无需额外重型运行时

---

## 后续可扩展方向（未在当前 MVP 实现）

- 可选加密存储（如系统密钥链集成）
- `import/export`（便于迁移配置）
- 配置校验模板（必填键检查）
- `run --print-env` 等调试辅助
- 项目别名与批量操作

---

## 开发与调试

运行 CLI：

```bash
cargo run -- list
cargo run -- show myproj
cargo run -- run myproj -- env
```

运行 GUI：

```bash
cargo run -- gui
```

代码检查：

```bash
cargo check
```

---

## 许可证

当前仓库未声明许可证。如需开源发布，请补充 `LICENSE` 文件并在此更新。
