# Woozzle

A tinker experiment in Rust and [Bevy](https://bevy.org). I wrote it to try out
a lot of entities, each with its own task pipeline, and to see how far I can get
on efficiency.

![Woozzle demo](doc/demo.gif)

[Full-quality video](doc/demo.mp4)

The task system design is in [doc/task_system.md](doc/task_system.md).

## Run

```sh
cargo run
cargo run --features dev  # faster incremental builds
```
