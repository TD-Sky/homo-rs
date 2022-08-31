# HOMO 论证器

一个可以将整数分解成**114514**算式的库。

本库是[itorr/homo](https://github.com/itorr/homo)的Rust移植，但没有分解小数功能。

## 使用

在`Cargo.toml`里添加：

```toml
[dependencies]
homo-rs = "0.1.0"
```

## 简易命令行

在项目里执行：

```bash
$ cargo run --bin cli -- <整数>
```

即可看到分解的结果。

## 许可证

MIT license ([LICENSE](./LICENSE) or https://opensource.org/licenses/MIT)
