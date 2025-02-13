## Rust Web服务器基础版
仅仅实现了 Api 请求和 数据库操作

#### start
```sh
## cargo install sqlx-cli

export DATABASE_URL="sqlite://todos.db"
cargo sqlx prepare
caogo run
```