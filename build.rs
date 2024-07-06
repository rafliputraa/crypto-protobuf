use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("protobuf");
    let proto_files = [
        PathBuf::from("proto/watchlist.proto").to_string_lossy().into_owned(),
    ];
    let proto_include = PathBuf::from("proto").to_string_lossy().into_owned();

    println!("Compiling proto files: {:?}", proto_files);
    println!("Proto include: {}", proto_include);
    println!("Output directory: {:?}", out_dir);

    prost_build::Config::new()
        .out_dir(out_dir.clone())
        .compile_protos(&proto_files, &[proto_include])
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));

    // Verify if the generated files are in the expected directory
    let watchlist_output_path = out_dir.join("crypto.watchlist.rs");

    assert!(watchlist_output_path.exists(), "Generated file for crypto.watchlist.rs does not exist");
}