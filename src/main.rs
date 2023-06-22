mod http;
mod socket;


fn main() {
    let server = socket::server::Server::new(8080).unwrap();
    server.listen().unwrap()
}
