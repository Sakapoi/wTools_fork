<!-- {{# generate.module_header{} #}} -->

# Module :: wproc_macro
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleProcMacroToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleProcMacroToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/wproc_macro?color=e3e8f0&logo=docs.rs)](https://docs.rs/wproc_macro) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Tools for writing procedural macroses.

### Sample

<!-- {{# generate.module_sample{} #}} -->

```rust
use wproc_macro::*;

fn main()
{
  let code = qt!( core::option::Option< i8, i16, i32, i64 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = type_parameters( &tree_type, 0..=2 );
  got.iter().for_each( | e | println!( "{}", qt!( #e ) ) );
  // < i8
  // < i16
  // < i32
}
```

### To add to your project

```sh
cargo add wproc_macro
```
