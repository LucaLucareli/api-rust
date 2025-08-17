//https://www.tabnews.com.br/ddanielsantos/criando-uma-api-rest-com-rust

use std::net::ToSocketAddrs;
use axum::{routing::get, Router};
use hyper::{Request};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::tokio::TokioIo;
use tokio::net::TcpListener;
use tower_service::Service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "localhost:8000"
        .to_socket_addrs()?
        .next()
        .expect("Erro ao resolver localhost");

    let listener = TcpListener::bind(addr).await.unwrap();

    let app = Router::new().route("/", get(|| async { "Hello, Axum!" }));
    let make_service = app.into_make_service();

    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);
        let mut make_service = make_service.clone();

        tokio::spawn(async move {
            // Cria o serviço para esta conexão
            let service = make_service.call(()).await.unwrap();

            // Usa `service_fn` para servir as requisições
            let handler = service_fn(move |req: Request<_>| {
                let mut svc = service.clone();
                async move {
                    // Infallible porque Axum não falha
                    svc.call(req).await.map_err(|_err| std::io::Error::new(std::io::ErrorKind::Other, "handler failed"))
                }
            });

            if let Err(err) = http1::Builder::new()
                .serve_connection(io, handler)
                .await
            {
                eprintln!("server error: {}", err);
            }
        });
    }
}
