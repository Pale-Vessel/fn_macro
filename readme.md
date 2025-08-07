## Macro for function traits
This crate adds a macro to derive the three function traits `FnOnce`, `FnMut`, and `Fn`. These traits are often implemented three times with the same signature and body (excluding the different borrow type on `self`), which can lead to unnecessary boilerplate - this macro hopes to overcome this.

## Usage
This macro is comprised of four parts - the initial `derive`, and three attributes for the input and output types, and the function body.

```toml
[dependencies]
fn_macro = "0.1.0"
```

```rust
#![feature(unboxed_closures)]
#![feature(fn_traits)]

use fn_macro::{Fn, fn_args, fn_body, fn_output};

#[derive(Fn)]
#[fn_args(f64, f64, String)]
#[fn_body{
    let k = self.0 + args.0;
    format!("{} {}", args.2, k + args.1)
}]
#[fn_output(String)]
struct Test(f64);

fn main() {
    let object = Test(9.5);
    println!("{}", object(1.0, 2.5, String::from("Hello"))) //Hello 13.0
}
```

## Known issues
Due to the use of `expect` in the macro code, VSCode will highlight the macro's use as incorrect code, claiming it will always panic. This is wrong - the macro will only panic if one of the necessary fields is not provided.