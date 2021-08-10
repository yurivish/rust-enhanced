# Tips

Some tips for working with Rust in Sublime Text:

* Use the Goto commands to quickly navigate the code.
  These commands make use of the syntax highlighting to detect symbols that are important, such as function names, type definitions, etc.
  These commands are:

  | Command | Keybind | Description |
  | ------- | ------- | ----------- |
  | Goto Symbol | Ctrl-R / ⌘-R | Jump to a symbol in the current file. |
  | Goto Symbol in Project | Ctrl-Shift-R / ⌘-Shift-R | Jump to a symbol in the current project. |
  | Goto Definition | F12 | Jump to the definition of the symbol currently under the cursor. |

* Use [Sublime Projects](https://www.sublimetext.com/docs/projects.html).
  This can help organize different projects, remembers the state of the window, and allows you to set custom settings for a specific project.

* When using on-save checking, or running a build, use the next/previous message keybindings to jump between the messages.
  See [Build usage](build/index.md#usage) for more.

* When running Cargo tests with the [test build command](build/index.md), if a test fails, the Next/Previous message commands should jump to the line where the test failed.

* Several [snippets](https://github.com/rust-lang/rust-enhanced/tree/master/snippets) are provided, which are simple patterns that are triggered with auto-completion.
  You can even write your own [custom snippets](https://docs.sublimetext.io/guide/extensibility/snippets.html).
