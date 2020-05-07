# lmake_delete_old_js_snippets

The old folders for js snippets are not automatically deleted on building with wasm-pack.  
This utils do that.
The util exe must be executed in the root project folder where is the cargo.toml.  
No arguments needed to execute the exe.  

## Makefile.toml for cargo make

In `Makefile.toml` for `cargo make` add a call like this:  

```toml
[tasks.dev]
description = "cargo build development"
clear = true
dependencies = [
    "lmake_version_from_date",
    "build_dev",
    "delete_old_js_snippets",
    "copy_to_webfolder",
]

[tasks.delete_old_js_snippets]
description = "delete old snippets"
clear = true
private = true
script= ["../../utils_linux/lmake_delete_old_js_snippets"]
```

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://bestia.dev/cargo_crev_web/query/num-traits>  
