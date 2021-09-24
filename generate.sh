#!/usr/bin/bash

set -e

cd sys
cargo run --release --manifest-path ../gir/Cargo.toml
sed -i 's/: \*mut _ESource/: \*mut ESource/g' src/lib.rs # XXX?
sed -i 's/-> \*mut _ESourceCredentialsProvider/-> \*mut ESourceCredentialsProvider/' src/lib.rs ### XXX?
sed -i 's/libxml::bindings_sys/libxml::bindings/' src/lib.rs
sed -i '/e_source_camel_configure_service/d' src/lib.rs
sed -i '/e_source_camel_get_settings/d' src/lib.rs
cd ..
cargo run --release --manifest-path gir/Cargo.toml
