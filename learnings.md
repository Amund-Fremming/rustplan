# Learnings

## Middleware

For more than one middleware pass inn `ServiceBuilder`.
For passing in middleware use `from_fn` to wrap the async func into the `tower::Layer` type.
For passing inn app state into your custom middleware use `from_fn_with_state(app_state), <your_middleware>`.

- _Could be useful for validating user requests and so on with returning a `Result<x, StatusCode>`._

## Code layers and async programming

Rust handles layers differently. We typically use a shared async state holding resources like database pools, which handlers can access directly.
Instead of bundling functions into service or repository objects, Rust passes the database pool from the shared app state into async functions,
keeping things efficient and avoiding unnecessary async complexity along the way.
The state is often an async-safe object wrapped in Arc, allowing cheap cloning of references without duplicating the data.
This lets us share the same state instance across multiple routes in our API.

## sqlx

In sqlx we mainly use two types of macros for queriying: - `sqlx::query!(...)` when we want to get some columns from some data,
and we let sqlx handle the construction of structs. For querying data into the structs we create we use `sqlx::query_as!(...)`.
TIP: never use string concatenation to create queries.
TIP: Insert, update, and delete needs to use `execute(db_pool)`

```rust
let row = sqlx::query!("SELECT id FROM users WHERE email = $1", email)
    .fetch_one(&pool)
    .await?;
```

```rust
let user = sqlx::query_as!(
    User,
    r#"
    SELECT id, email, is_active
    FROM users
    WHERE email = $1
    "#,
    email
)
.fetch_optional(pool)
.await?;
```

Sqlx also has support for some different fetch types:

- `fetch_one` used to get one item as a result.
- `fetch_many` used to get a vec of items as a result.
- `fetch_optional` used to get a item as a option packaged inside a result.
- `fetch_optional` used to get a item as a option packaged inside a result.
- `fetch` used to fetch results as a stream, used for large datasets when we dont want to load the whole set into memory

## sqlx migrations

Start of by installing the cli `cargo install sqlx-cli`.