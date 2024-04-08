# rust_dylibs_poc

Some tests to work out how to create and use dynamic librairies in Rust, and if we should ?

## Context

This work was made as part of a Stormshield R&D project.

![Stormshield Logo](https://www.stormshield.com/wp-content/uploads/stormshield-logo.png)

## Hypothesis

The tested hypothesis are on git branches :
- simple : simple working use case without async
- pass_handle_to_remote : we try to pass a tokio handle to the library ; that doesn't work
- remote_future : we try to export a future from the library and run it in the main program ; that doesn't work
- tokios : we use two tokios runtimes ; that works