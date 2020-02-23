re: https://github.com/pyros2097/rust-embed/issues/99

Note that `report` is in a cargo workspace and references `assets/public/`.

You get different results depending on _where_ you run `cargo run`/`cargo run --release`.

When run in the `WORKSPACE_ROOT`, `WORKSPACE_ROOT\assets\public\an-image.jpg` is used (cat). When run in `WORKSPACE_ROOT\report` the `WORKSPACE_ROOT\reports\assets\public\an-image.jpg` is used (dog).