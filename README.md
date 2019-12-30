Minimal example of https://github.com/tokio-rs/tracing/issues/495

## Output

```rust
=== good ===
Dec 29 16:48:48.500 ERROR tracing_bug_poc: ++ span1;
Dec 29 16:48:48.500 ERROR span1{}: tracing_bug_poc: -> span1
Dec 29 16:48:48.500 ERROR span1{}: tracing_bug_poc: ++ span2;
Dec 29 16:48:48.500 ERROR span1{}:span2{}: tracing_bug_poc: -> span2
Dec 29 16:48:48.500 ERROR span1{}: tracing_bug_poc: <- span2
[tracing-subscriber/src/layer.rs:188] format!("closing span {:?}", id) = "closing span Id(2)"
Dec 29 16:48:48.501 ERROR span1{}: tracing_bug_poc: -- span2
Dec 29 16:48:48.501 ERROR tracing_bug_poc: <- span1
[tracing-subscriber/src/layer.rs:188] format!("closing span {:?}", id) = "closing span Id(1)"
Dec 29 16:48:48.501 ERROR tracing_bug_poc: -- span1

=== bad ===
Dec 29 16:48:48.501 ERROR tracing_bug_poc: ++ span1;
Dec 29 16:48:48.501 ERROR span1{}: tracing_bug_poc: -> span1
Dec 29 16:48:48.501 ERROR span1{}: tracing_bug_poc: ++ span2;
Dec 29 16:48:48.501 ERROR span1{}:span2{}: tracing_bug_poc: -> span2
Dec 29 16:48:48.501 ERROR span1{}:span2{}: tracing_bug_poc: <- span1
Dec 29 16:48:48.501 ERROR span1{}:span2{}: tracing_bug_poc: -- span1
Dec 29 16:48:48.501 ERROR span1{}: tracing_bug_poc: <- span2
[tracing-subscriber/src/layer.rs:188] format!("closing span {:?}", id) = "closing span Id(4)"
Dec 29 16:48:48.501 ERROR span1{}: tracing_bug_poc: -- span2
```
