# Rust Enhanced

## About

Rust Enhanced is a [Sublime Text](https://www.sublimetext.com/) package which adds extended support for the [Rust Programming Language](https://www.rust-lang.org/).
This is a replacement for the built-in "Rust" package and provides several extensions:

* Enhanced syntax highlighting which handles more recent language changes and improved highlighting.
* A custom build system with complete configuration of running Cargo and supporting Rust's extended error messages.
* Automatic checking every time you save a file.
* Custom highlighting for Cargo output.

<img src="docs/img/running_tests.gif?raw=true" alt="Running Tests with Rust Enhanced" width=430 style="margin-right:10px"> <img src="docs/img/showing_errors.gif?raw=true" alt="Highlighting errors and warnings with Rust Enhanced" width=430>

## Installation and Usage

See the [**Rust Enhanced User Guide**](https://rust-lang.github.io/rust-enhanced/) for complete information on installing and using this package.

## Contributing

Development is quite simple, first uninstall "Rust Enhanced" if you already have it installed.
Then, check out this project to your Sublime Text `Packages` folder.

Syntax definitions are defined in the [`RustEnhanced.sublime-syntax`](RustEnhanced.sublime-syntax) file, and syntax tests are in the [`tests/syntax-rust`](tests/syntax-rust) directory.

The [PackageDev](https://packagecontrol.io/packages/PackageDev) package is highly recommended for doing development.

## Credits

Created 2012 by [Daniel Patterson](mailto:dbp@riseup.net), as a near complete from
scratch rewrite of a package by [Felix Martini](https://github.com/fmartini).

This project is currently maintained by [Eric Huss](https://github.com/ehuss)

## License

This package is licensed under the MIT License.
