use std::{path::PathBuf, env};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_client(true)
        .out_dir(out_dir)
        .compile(&["proto/arg.proto", "proto/audit_info.proto", "proto/basic.proto", "proto/block.proto", "proto/broker.proto", "proto/chain.proto",
         "proto/ibtp.proto", "proto/interchain_meta.proto", "proto/receipt.proto", "proto/transaction.proto"], &["proto"])
        .unwrap();
}