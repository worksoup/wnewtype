# Worksoup's newtype
**fork 自 crate [`newtype`](https://gitlab.com/jrobsonchase/newtype) v0.2.1.**

为结构体实现 `newtype` 模式。这将为内含值实现 `From`, `Into`, `Deref` 和 `DerefMut` 特型。

异于原项目：
- 不仅作用于元组结构体；
- 字段数量任意，通过 `#[inner]` 字段属性确定内部类型，其余均使用 `Default` 特型进行初始化。

原介绍：

> Treat a single-field tuple struct as a "newtype"
>
> This will implement `From`, `Into`, `Deref`, and `DerefMut` for the inner
> type.

原介绍翻译：

> 为单字段元组结构体实现 `newtype` 模式。
>
> 这将为内含值实现 `From`, `Into`, `Deref` 和 `DerefMut` 特型。


# LICENSE
见[LICENSE.md](./LICENSE.md)
