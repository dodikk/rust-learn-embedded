
## https://os.phil-opp.com/minimal-rust-kernel/


# export XARGO_RUST_SRC=/usr/local/Cellar/rust/1.26.2/share/rust/rust_src/




# == once == 
#
# cargo install cargo-xbuild
# rustup component add rust-src
#
#
# cargo xbuild --target x86_64-blog_os.json


## `cargo xbuild` used under the hood
#
# bootimage build --target x86_64-blog_os.json


## `bootimage build` and `cargo xbuild` under the hood
#
bootimage build
