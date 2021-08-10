# On-save checking

Rust Enhanced will automatically run `cargo check` every time you save a `.rs` source file.
A small message will be shown in the status bar to indicate that the check is running.
If there are any errors or warnings, they will be displayed based on the settings described in the [Diagnostic Messages](build/messages.md) chapter.

## On-save settings

There are a few settings for controlling the on-save behavior.
See the [Settings](settings.md) chapter for more on how to configure these.

| Setting | Default | Description |
| :------ | :------ | :---------- |
| `rust_syntax_checking` | `true` | Enable the on-save syntax checking. |
| `rust_syntax_checking_method` | `"check"` | The method used for checking your code (see below). |
| `rust_syntax_checking_include_tests` | `true` | Enable checking of test code within `#[cfg(test)]` sections. |

The available checking methods are:

| Method | Description |
| :----- | :---------- |
| `check` | Uses `cargo check` (requires at least Rust 1.16). |
| `clippy` | Uses `cargo clippy`. This requires [Clippy](https://github.com/rust-lang/rust-clippy) to be installed. This also may be a little slower since it must check every target in your package. |

This will use the same configuration options as the "Check" and "Clippy" build variants (for example, extra environment variables, or checking with different features).
See [the build docs](build/index.md) for more information.

If a Cargo project has several build targets (`--lib`, `--bin`, `--example`, etc.), it will attempt to automatically detect the correct target, and only check that target.

In some rare cases, you may need to manually specify which target a file belongs to.
This can be done by adding a "projects" setting in `RustEnhanced.sublime-settings` with the following format:

```
    "projects": {
       // One entry per project. Keys are project names.
       "my_cool_stuff": {
           // Path to the project root dir without trailing /src.
           "root": "/path/to/my/cool/stuff/project",

           // Targets will be used to replace {target} part in the command.
           // If no one target matches an, empty string will be used.
           "targets": {
               "bin/foo.rs": "--bin foo",  // format is "source_code_filename -> target_name"
               "bin/bar.rs": "--bin bar",
               "_default": "--lib"         // or "--bin main"
           }
       }
   }
```
