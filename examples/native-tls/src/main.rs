use salvo::listener::native_tls::{NativeTlsConfig, NativeTlsListener};
use salvo::prelude::*;

#[handler]
async fn hello_world(res: &mut Response) {
    res.render(Text::Plain("Hello World"));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello_world);
    let listener = NativeTlsListener::with_config(
        NativeTlsConfig::new()
            .with_pkcs12(include_bytes!("../certs/identity.p12").to_vec())
            .with_password("mypass"),
    )
    .bind("127.0.0.1:7878");
    tracing::info!("Listening on https://127.0.0.1:7878");
    Server::new(listener).serve(router).await;
}
