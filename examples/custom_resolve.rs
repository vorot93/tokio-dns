use tokio_dns::CpuPoolResolver;

#[tokio::main]
async fn main() {
    // create a custom, 10 thread CpuPoolResolver
    let resolver = CpuPoolResolver::new(10);

    match tokio_dns::resolve_sock_addr_with("rust-lang.org:80", resolver.clone()).await {
        Ok(addrs) => println!("Socket addresses {:#?}", addrs),
        Err(err) => println!("Error resolve address {:?}", err),
    }
}
