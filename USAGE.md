# gitb.exe 使用说明

下载 `gitb.exe` 后的安装与使用指南（Windows）。

## 一、安装

### 1. 放置可执行文件

把 `gitb.exe` 放到一个固定目录，例如：

```
C:\Tools\gitb\gitb.exe
```

### 2. 加入 PATH（推荐）

> 加入 PATH 后，任意目录下都能直接敲 `gitb`，不用每次写完整路径。

#### 方法 A：图形界面（适合所有用户）

1. `Win + R` → 输入 `sysdm.cpl` → 回车
2. **高级** 选项卡 → **环境变量**
3. 在「用户变量」中找到 `Path` → **编辑**
4. **新建** → 粘贴目录路径：`C:\Tools\gitb`
5. 一路 **确定** 保存

#### 方法 B：PowerShell 永久添加（仅当前用户）

```powershell
# 把 C:\Tools\gitb 永久加入当前用户的 PATH
[Environment]::SetEnvironmentVariable(
  "Path",
  [Environment]::GetEnvironmentVariable("Path", "User") + ";C:\Tools\gitb",
  "User"
)
```

#### 方法 C：临时添加（仅当前窗口生效）

```powershell
$env:Path += ";C:\Tools\gitb"
```

> ⚠️ 三种方法任选其一。改完后**重开** PowerShell / CMD 窗口才会生效。
> 验证是否生效：`where.exe gitb` 应能输出 exe 路径。

### 3. 验证安装

```powershell
gitb --version
```

看到版本号即成功。看不到则检查 PATH 是否生效、目录是否正确。

## 二、快速上手

> **工作区概念**：gitb 操作的是「当前目录下的所有 Git 仓库」。先把多个仓库放到同一个父目录里，再 `cd` 进去运行 gitb。
>
> 例如目录结构：
> ```
> D:\Work\
> ├── repo-a\    (.git)
> ├── repo-b\    (.git)
> └── repo-c\    (.git)
> ```
> 在 `D:\Work\` 下运行 `gitb status` 就会同时扫描 repo-a / repo-b / repo-c。

```powershell
# 查看当前目录下所有仓库的状态（分支 / 是否有改动 / 领先落后情况）
gitb status

# 并行拉取所有仓库的更新（8 个并发，仓库多时明显更快）
gitb pull -j 8

# 在所有仓库中切换到 main 分支（支持模糊匹配，如 main / master 都能命中）
gitb checkout main

# 创建并切换到新分支（所有仓库一起切）
gitb create feature/x

# 推送所有仓库
gitb push

# 健康检查：哪些仓库落后、有未提交、有未推送 —— 上班第一件事跑一遍
gitb doctor
```

> 默认只扫描当前目录的直接子目录（深度 1）。要扫更深，用 `-d 2`。

## 三、常用命令速查

| 命令 | 作用 | 备注 |
|------|------|------|
| `gitb status` | 多仓库状态总览 | 入门第一命令，先看状态再操作 |
| `gitb pull` | 拉取并合并远程更新 | 日常同步用，`-j 8` 提速 |
| `gitb fetch` | 仅获取远程引用，不合并 | 只想看远程有啥变化时用 |
| `gitb push` | 推送本地提交到远程 | 推送前建议先 `gitb status` 确认 |
| `gitb checkout <分支>` | 切换分支（模糊匹配） | 输入 `main` 可匹配到 `origin/main` 等 |
| `gitb create <分支>` | 新建并切换分支 | 所有仓库统一建分支，适合并行开发 |
| `gitb commit -m "msg"` | 提交更改 | `-a` 同时 `git add -A`，省一步 |
| `gitb stash push` | 暂存当前改动 | 临时切走前先 stash |
| `gitb stash pop` | 弹出最新暂存 | 回到改动 |
| `gitb stash list` | 列出各仓库暂存 | 查看有哪些 stash |
| `gitb stash clear` | 清空所有暂存 | 谨慎，不可恢复 |
| `gitb rebase` | 智能变基 | 自动 stash→rebase→unstash，脏仓库也能用 |
| `gitb rebase -b main` | 变基到指定分支 | 统一拉到 main 上 |
| `gitb diff` | 查看所有仓库差异 | 代码评审前快速扫一遍 |
| `gitb log -n 10` | 每个仓库最近 N 条提交 | 默认 5 条，`-n` 调数量 |
| `gitb branch list` | 列出所有仓库分支 | 查看分支分布 |
| `gitb branch delete <name>` | 删除分支 | `-f` 强删，`--remote` 连远程一起删 |
| `gitb exec <git子命令>` | 执行任意 git 命令 | 万能口，如 `gitb exec log --oneline -5` |
| `gitb doctor` | 健康检查 | 查哪些仓库落后/脏/未推送，每周跑一次 |

### 典型工作流

**每日同步：**
```powershell
gitb status          # 先看状态
gitb pull -j 8       # 并行拉取
gitb doctor          # 检查有无异常
```

**统一切分支开发：**
```powershell
gitb create feature/x   # 所有仓库统一建分支
# ...开发...
gitb commit -am "feat: do something"
gitb push
```

**清理旧分支：**
```powershell
gitb branch list
gitb branch delete old-branch -f --remote
```

## 四、全局选项

| 选项 | 说明 |
|------|------|
| `-j N` | 并行任务数（0=自动按 CPU 核数） |
| `-g <组名>` | 只对某分组仓库执行 |
| `-s dir1,dir2` | 跳过指定目录 |
| `-d N` | 仓库发现最大递归深度（默认 1） |
| `-o table\|json\|quiet` | 输出格式 |
| `-f` | 强制操作（丢弃未提交更改） |
| `--dry-run` | 只显示会做什么，不实际执行 |
| `-v` / `-q` | 详细 / 静默 |

## 五、分组管理

把仓库分组，方便只操作一部分：

```powershell
# 新增分组
gitb group add frontend repo-a,repo-b,repo-c

# 查看分组
gitb group list

# 只对 frontend 分组执行 pull
gitb pull -g frontend

# 删除分组
gitb group remove frontend
```

## 六、配置文件（可选）

在工作区根目录放 `gitb.toml` 即可持久化配置。零配置也能用。

```toml
[workspace]
default_branch = "main"
default_skip = ["node_modules", "target"]
default_depth = 2

[groups.frontend]
repos = ["web-app", "mobile-app"]

[groups.backend]
repos = ["api-gateway", "user-service"]
```

或用交互式向导生成：

```powershell
gitb init
```

## 七、PowerShell 补全（可选）

把补全脚本写进 PowerShell 配置文件，重开终端后即可 Tab 补全：

```powershell
# 查看 $PROFILE 路径
echo $PROFILE

# 追加补全脚本
gitb completion powershell >> $PROFILE

# 立即生效
. $PROFILE
```

## 八、常见问题

**Q: 提示「gitb 不是内部或外部命令」？**
A: PATH 未生效。确认目录已加入 PATH，并**重开**终端窗口。

**Q: 扫不到仓库？**
A: 默认深度 1。仓库在更深层级时用 `-d 2` 或 `-d 3`。

**Q: 命令卡住 / 太慢？**
A: 用 `-j` 提高并发，或 `-q` 减少输出。仓库很多时并发能显著提速。

**Q: 不确定会改什么？**
A: 先加 `--dry-run` 预览，确认无误再去掉。

**Q: 想跳过某些目录？**
A: `-s node_modules,target,.vscode` 逗号分隔。
