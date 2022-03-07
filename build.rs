fn main() {
    tonic_build::configure()
        .build_client(true)
        .out_dir("src/pb")        
        .compile(&["proto/arg.proto", "proto/audit_info.proto", "proto/basic.proto", "proto/block.proto", "proto/broker.proto", "proto/chain.proto",
         "proto/ibtp.proto", "proto/interchain_meta.proto", "proto/receipt.proto", "proto/transaction.proto"], &["proto"])
        .unwrap();
}