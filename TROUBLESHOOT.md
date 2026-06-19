# TROUBLESHOOT
## Tests are not compiling
### Symptoms: error[E0463]: can't find crate for `std`
```
$ cargo test --lib --target=x86_64-unknown-linux-gnu
   Compiling compiler_builtins v0.1.153
   Compiling core v0.0.0 (C:\Users\Emmanuel\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core)
   Compiling rustc-std-workspace-core v1.99.0 (C:\Users\Emmanuel\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\rustc-std-workspace-core)
   Compiling gba-playground v0.0.0 (D:\DataEmmanuel\Programmation\0V3RG4M3\gba-playground)                                                                   
error[E0463]: can't find crate for `std`                                                                                                                     
  |
  = note: the `x86_64-unknown-linux-gnu` target may not support the standard library
  = note: `std` is required by `gba_playground` because it does not declare `#![no_std]`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`

error[E0463]: can't find crate for `test`                                                                                                                    

error: cannot find attribute `test` in this scope                                                                                                            
  --> src\scene.rs:30:7
   |
30 |     #[test]
   |       ^^^^

error: cannot find macro `assert_eq` in this scope                                                                                                           
  --> src\scene.rs:74:13
   |
74 |             assert_eq!(context, expected_context);
   |             ^^^^^^^^^
   |
help: consider importing this macro
   |
28 +     use core::assert_eq;
   |

error: `#[panic_handler]` function required, but not found                                                                                                   

error: unwinding panics are not supported without std
  |
  = help: using nightly cargo, use -Zbuild-std with panic="abort" to avoid unwinding
  = note: since the core library is usually precompiled with panic="unwind", rebuilding your crate with panic="abort" may not be enough to fix the problem

For more information about this error, try `rustc --explain E0463`.                                                                                          
error: could not compile `gba-playground` (lib test) due to 6 previous 
``` 
### Solution: Use correct target
Try one of these:
```bash
cargo test --lib --target=x86_64-unknown-linux-gnu
```
```bash
cargo test --lib --target=x86_64-pc-windows-msvc
```