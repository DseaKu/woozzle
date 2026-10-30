# Woozzle

A tinker experiment in Rust and [Bevy](https://bevy.org). I wrote it to try out
a lot of entities, each with its own task pipeline, and to see how far I can get
on efficiency.

![Woozzle demo](doc/demo.gif)

In the demo above, all entities execute a sequential task queue with active
collision detection: they start by roaming randomly, then transition to
patrolling a rectangular path, and finally return to random roaming.

## Run

```sh
cargo run
cargo run --features dev  # faster incremental builds
```
