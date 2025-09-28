# Rust 工作空间示例

这是一个 Rust 工作空间（Workspace）的示例项目，展示了如何在单个工作空间中管理多个相关的 crate。

## 项目结构

```
workspace/
├── Cargo.toml          # 工作空间配置文件
├── README.md           # 项目文档
├── add_one/            # 加一库
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── add_two/            # 加二库
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── adder/              # 演示程序
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## 包含的 Crate

### 1. add_one

一个简单的数学工具库，提供将整数加一的功能。

**功能特性：**
- 安全的整数加一操作
- 完整的测试覆盖
- 详细的文档注释

**使用示例：**
```rust
use add_one::add_one;

let result = add_one(5);
assert_eq!(result, 6);
```

### 2. add_two

一个数学工具库，提供将整数加二的功能，内部使用 `add_one` 库实现。

**功能特性：**
- 安全的整数加二操作
- 展示库之间的依赖关系
- 完整的测试覆盖
- 详细的文档注释

**使用示例：**
```rust
use add_two::add_two;

let result = add_two(5);
assert_eq!(result, 7);
```

### 3. adder

一个演示程序，展示如何在同一工作空间中使用多个库 crate。

**功能：**
- 演示工作空间中库之间的依赖关系
- 展示如何导入和使用外部 crate
- 提供简单的数学运算示例

## 如何使用

### 构建整个工作空间

```bash
cargo build
```

### 运行演示程序

```bash
cargo run -p adder
```

### 运行特定包的测试

```bash
# 测试 add_one 库
cargo test -p add_one

# 测试 add_two 库
cargo test -p add_two

# 运行所有测试
cargo test
```

### 生成文档

```bash
# 生成所有包的文档
cargo doc

# 生成并打开文档
cargo doc --open
```

### 发布包

```bash
# 发布特定的包
cargo publish -p add_one
cargo publish -p add_two
```

## 工作空间的优势

1. **统一管理**：所有相关的 crate 在同一个仓库中管理
2. **共享依赖**：工作空间中的所有 crate 共享 `Cargo.lock` 文件
3. **简化构建**：可以一次性构建所有 crate
4. **版本一致性**：确保所有 crate 使用相同版本的依赖
5. **开发效率**：在开发过程中可以轻松地在 crate 之间进行更改

## 依赖关系图

```
adder
├── add_one
└── add_two
    └── add_one
```

## 学习要点

这个示例项目展示了以下 Rust 概念：

- **工作空间配置**：如何设置 `Cargo.toml` 文件
- **包依赖**：如何在工作空间内引用其他包
- **库设计**：如何创建可重用的库 crate
- **文档编写**：如何编写高质量的文档注释
- **测试组织**：如何为库编写和组织测试

## 扩展建议

可以考虑添加以下功能来进一步学习：

1. 添加更多的数学运算库（subtract_one, multiply_two 等）
2. 创建一个通用的数学运算 trait
3. 添加错误处理和边界检查
4. 实现更复杂的依赖关系
5. 添加基准测试（benchmarks）
6. 集成 CI/CD 流水线

## 许可证

本项目仅用于学习目的。
