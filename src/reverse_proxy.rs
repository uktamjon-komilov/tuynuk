use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use clap::Parser;
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
use hyper::body::Incoming;

#[derive(Parser, Debug)]
struct Cli {
    /// Address to listen on (e.g. 127.0.0.1:3000)
    #[arg(long, default_value = "127.0.0.1:3000")]
    listen: String,

    /// Upstream backend servers
    #[arg(long = "backend")]
    backends: Vec<String>,
}

struct State {
    backends: Vec<String>,
    counter: Mutex<usize>,
    client: Client<HttpConnector, Body>,
}

impl State {
    fn new(backends: Vec<String>) -> Self {
        Self {
            backends,
            counter: Mutex::new(0),
            client: Client::new(),
        }
    }

    fn next_backend(&self) -> String {
        let mut idx = self.counter.lock().unwrap();
        let backend = self.backends[*idx % self.backends.len()].clone();
        *idx += 1;
        backend
    }
}

async fn proxy(state: Arc<State>, req: Request<Incoming>) -> Result<Response<Body>, hyper::Error> {
    let backend = state.next_backend();

    let (mut parts, body) = req.into_parts();
    let path = parts
        .uri
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("/");

    let uri: Uri = format!("http://{}{}", backend, path).parse().unwrap();
    parts.uri = uri;
    let new_req = Request::from_parts(parts, Body::from(body));

    state.client.request(new_req).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Cli::parse();

    if args.backends.is_empty() {
        eprintln!("At least one --backend must be specified");
        std::process::exit(1);
    }

    let addr: SocketAddr = args.listen.parse()?;
    let state = Arc::new(State::new(args.backends));

    println!("Reverse proxy listening on http://{}", addr);

    let make_svc = make_service_fn(move |_| {
        let state = state.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| proxy(state.clone(), req)))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    server.await?;

    Ok(())
}
