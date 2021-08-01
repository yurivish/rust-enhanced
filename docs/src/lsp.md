# Language Servers

The Rust Enhanced package does not currently provide any assistance with setting up a [Language Server](https://microsoft.github.io/language-server-protocol/).
Language Servers are processes that run in the background and provide support for a wide range of syntax analysis, code refactoring, and much more.

There are two language server implementations for Rust:

* [RLS (Rust Language Server)](https://github.com/rust-lang/rls)
* [rust-analyzer](https://rust-analyzer.github.io/)

RLS is being replaced by rust-analyzer, and you it is recommended to use rust-analyzer at this time.

To use one of the language servers, you need to install the [Sublime LSP](https://github.com/sublimelsp/LSP) plugin.
With both the plugin and one of the above servers installed, you should be able to follow the [LSP docs](https://lsp.readthedocs.io/en/latest/) for how to configure the server.
Generally this involves running either the `LSP: Enable Language Server Globally` or `LSP: Enable Language Server in Project` and then selecting either `rls` or `rust-analyzer`.
Depending on the size of your project, it may take some time for it to process and index.

Note that as well as error checking, code-completion, and renaming, RLS can run [`rustfmt`](https://github.com/rust-lang/rustfmt) on your code: right-click, and select `LSP > Format Document` or `Format Selection` in a Rust source file.
