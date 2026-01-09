# 发布到 crates.io 检查清单

## ✅ 已完成项

### 1. 基础文件
- [x] `Cargo.toml` - 包含所有必要的元数据
- [x] `README.md` - 中文文档（完整）
- [x] `README.en.md` - 英文文档（完整）
- [x] `LICENSE` - MIT 许可证
- [x] `CHANGELOG.md` - 更新日志

### 2. Cargo.toml 元数据
- [x] `name` = "tjpgdec-rs"
- [x] `version` = "0.4.0"
- [x] `authors`
- [x] `edition` = "2021"
- [x] `license` = "MIT OR Apache-2.0"
- [x] `description` - 简洁的描述
- [x] `repository` - GitHub 仓库地址
- [x] `homepage` - 项目主页
- [x] `documentation` - docs.rs 链接
- [x] `readme` = "README.md"
- [x] `keywords` - 5 个关键词
- [x] `categories` - 分类标签

### 3. 文档质量
- [x] README 包含使用示例
- [x] README 包含安装说明
- [x] README 包含 API 文档
- [x] 示例代码可运行
- [x] 双语文档（中文+英文）

### 4. 代码质量
- [x] 所有示例在 `examples/` 目录
- [x] 通过 `cargo test`
- [x] 通过 `cargo build`
- [x] 通过 `cargo clippy`

## 📝 发布前建议

### 必做事项
1. **验证 GitHub 仓库**
   - 确保仓库地址正确：https://github.com/planet0104/tjpgdec-rs
   - 如果仓库不存在，创建新仓库
   - 确保仓库是公开的

2. **运行发布前检查**
   ```bash
   # 检查包内容
   cargo package --list
   
   # 本地测试打包
   cargo package
   
   # 检查打包后的文件
   cargo package --allow-dirty
   
   # 测试打包的 crate
   cargo publish --dry-run
   ```

3. **版本标签**
   - 在 Git 中创建版本标签：
   ```bash
   git tag -a v0.4.0 -m "Release version 0.4.0"
   git push origin v0.4.0
   ```

### 可选优化

1. **添加徽章到 README**
   ```markdown
   [![Crates.io](https://img.shields.io/crates/v/tjpgdec-rs.svg)](https://crates.io/crates/tjpgdec-rs)
   [![Documentation](https://docs.rs/tjpgdec-rs/badge.svg)](https://docs.rs/tjpgdec-rs)
   [![License](https://img.shields.io/crates/l/tjpgdec-rs.svg)](https://github.com/planet0104/tjpgdec-rs#license)
   ```

2. **创建 GitHub Release**
   - 在 GitHub 上创建 Release
   - 附上 CHANGELOG 内容

3. **添加 .gitignore**
   - 确保不包含不必要的文件

## 🚀 发布步骤

1. **最终检查**
   ```bash
   cargo test
   cargo clippy
   cargo doc --open  # 检查生成的文档
   ```

2. **发布到 crates.io**
   ```bash
   # 登录 crates.io (首次)
   cargo login
   
   # 发布
   cargo publish
   ```

3. **发布后验证**
   - 访问 https://crates.io/crates/tjpgdec-rs
   - 检查文档 https://docs.rs/tjpgdec-rs
   - 测试安装：`cargo install tjpgdec-rs`

## ⚠️ 注意事项

1. **版本号规则**
   - 遵循语义化版本 (SemVer)
   - 当前版本：0.4.0
   - 下次更新根据变更类型递增

2. **发布是永久的**
   - crates.io 不允许删除已发布的版本
   - 只能发布补丁版本（yank）

3. **文件大小限制**
   - 单个 crate 最大 10MB
   - 使用 `.cargo_vcs_info.json` 排除不必要的文件

4. **依赖版本**
   - 确保依赖版本合理
   - 当前依赖：heapless = "0.8"

## 📋 GitHub 仓库描述建议

**Description (简短描述):**
```
Tiny JPEG Decoder for embedded systems - Rust implementation of ChaN's TJpgDec
```

**Topics (话题标签):**
- rust
- jpeg
- decoder
- embedded
- no-std
- esp32
- image-processing
- embedded-systems
- tjpgdec

**About section:**
- Website: https://docs.rs/tjpgdec-rs
- Add README
- Add License: MIT
- Add topics (见上方)

## 🎯 最后确认

在执行 `cargo publish` 前，确认：
- [ ] 所有测试通过
- [ ] GitHub 仓库已创建并推送
- [ ] README 中的链接都正确
- [ ] 版本号正确
- [ ] CHANGELOG 已更新
- [ ] 没有包含敏感信息
