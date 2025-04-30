use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    let out_dir = format!("{}/protos", std::env::var("OUT_DIR").unwrap());

    std::fs::create_dir_all(&out_dir).unwrap();

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(out_dir)
        .inputs(["protos/rendezvous.proto", "protos/message.proto"])
        .include("protos")
        .customize(protobuf_codegen::Customize::default().tokio_bytes(true))
        .run()
        .expect("Codegen failed.");
    
    // --- Custom code ---
    println!("cargo:rerun-if-env-changed=HBB_RENDEZVOUS_SERVER");
    println!("cargo:rerun-if-env-changed=HBB_RS_PUB_KEY");
    let rendezvous_server = env::var("HBB_RENDEZVOUS_SERVER")
        .expect("HBB_RENDEZVOUS_SERVER environment variable must be set");
    let rs_pub_key = env::var("HBB_RS_PUB_KEY")
        .expect("HBB_RS_PUB_KEY environment variable must be set");
    let formatted_rendezvous_servers = format!("&[\"{}\"]", rendezvous_server);
    let formatted_rs_pub_key = format!("\"{}\"", rs_pub_key);
    let out_dir_main = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest_path_secrets = out_dir_main.join("generated_secrets.rs");
    let generated_secrets_code = format!("pub const RENDEZVOUS_SERVERS: &[&str] = {};\npub const RS_PUB_KEY: &str = {};\n", formatted_rendezvous_servers, formatted_rs_pub_key);
    fs::write(dest_path_secrets, generated_secrets_code)
        .expect("Failed to write generated_secrets.rs");
}
