# promql-rs
[PromQL](https://prometheus.io/docs/prometheus/latest/querying/basics/) parser via [Nom6](https://github.com/Geal/nom), which was inspired by [promql](https://crates.io/crates/promql) but redesign in nom function (no macros and leverage type system to debug)

# Install
```toml
promql = {git = "https://github.com/detailyang/promql-rs.git", branch = "main"}
```

# Example
```rust
    let (_, ast) = promql::parse_expr("min_over_time(rate(http_requests_total[5m])[30m:1m])").unwarp();
    println("ast: {:?}", ast);
```

# Declaration
This works on production but **Use at your own risk !!!**
