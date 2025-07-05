# Markst - Markdown与Typst的双向转换器

[![项目状态: 开发中](https://img.shields.io/badge/status-in%20development-yellow.svg)](https://github.com/YOUR_USERNAME/markst)
[![语言: Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org)
[![许可证: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

**Markst** 是一个旨在实现 [Markdown](https://www.markdownguide.org/) 和 [Typst](https://typst.app/) 之间高效、高保真度双向转换的工具。它让内容创作者可以享受 Markdown 的简洁与 Typst 的强大排版能力。

## 项目状态

:warning: **本项目正处于早期开发阶段。** 功能尚不完善，API 可能会发生变化。

## 核心功能

我们的目标是实现以下功能：

### Markdown -> Typst (`md2typ`) - 主要方向
- [ ] **基础语法转换**:
  - [ ] 标题 (`# H1`) -> `#heading[H1]`
  - [ ] 粗体/斜体/删除线
  - [ ] 列表（有序/无序）
  - [ ] 链接与图片
  - [ ] 代码块与行内代码
  - [ ] 引用块
  - [ ] 分割线
- [ ] **扩展功能 (GFM)**:
  - [ ] 表格转换
- [ ] **元数据处理**:
  - [ ] 解析 YAML Front Matter (title, author, date) 并转换为 Typst `set` 规则
- [ ] **数学公式**:
  - [ ] 支持 LaTeX 数学公式
- [ ] **模板支持**:
  - [ ] 允许用户指定一个 Typst 模板文件，将转换后的内容注入其中

### Typst -> Markdown (`typ2md`) - 尽力而为
- [ ] **基础结构提取**:
  - [ ] 标题、列表、链接等基础元素的转换
- [ ] **内容保留**:
  - [ ] 对于无法转换的 Typst 脚本和复杂布局，尽力提取其纯文本内容
- **注意**: 此方向的转换必然是**有损**的，主要用于内容提取和迁移，而非格式的完美还原。

## 安装与使用

**前提条件:** 你需要安装 [Rust 工具链](https://rustup.rs/)。

1.  **从源码构建 (当前阶段):**
    ```bash
    # 克隆仓库
    git clone https://github.com/forliage/Markst.git
    cd markst

    # 运行转换 (示例命令，待实现)
    cargo run -- md2typ "path/to/input.md" -o "path/to/output.typ"
    ```

## 开发路线图

1.  **阶段一 (MVP):** 实现核心的 `md2typ` CLI，支持所有基础 Markdown 语法。
2.  **阶段二 (功能增强):** 完善 `md2typ`，支持表格、YAML 等，并初步实现 `typ2md` 的内容提取。
3.  **阶段三 (健壮性):** 引入模板支持、完善的错误处理和全面的测试。
4.  **阶段四 (生态扩展):** 发布为库 (crate)、开发 VS Code 插件、探索 WebAssembly 版本。

## 贡献

欢迎任何形式的贡献！你可以：
- 提出 Issue 来报告 Bug 或建议新功能。
- Fork 本项目并提交 Pull Request。

## 许可证

本项目采用 [MIT](./LICENSE) 许可证。