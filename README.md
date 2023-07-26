# kyoto

tokio is useful for sending telegram msgs and stuff so this is handy

```rust
// example
if let Err(e) = res {
  // will do the thing in background for you
  // and leave your sync code alone
  kyoto::spawn(send_telegram_msg())
}
```
