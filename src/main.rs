use std::sync::Arc;
use tokio::sync::Mutex;
use salvo::prelude::*;
use shapoist_request::prelude::Server as ShapoistServer;
// use shapoist_request::prelude::*;


pub struct ServerTop {
	pub inner: Arc<Mutex<ShapoistServer>> 
}

#[handler]
impl ServerTop {
	async fn handle(&self, req: &mut Request) -> String {
		let request = req.param("request").unwrap();
		let server = self.inner.lock();
		let inner = server.await.handle_request_json(request);
		match serde_json::to_string(&inner) {
			Ok(inner) => inner,
			Err(_) => "unexpected error occurred".into()
		}
	}
}

#[handler]
async fn hello() -> &'static str {
	"Hello World"
}

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt().init();

	let mut server_inner = ShapoistServer::init().expect("init server failed");
	server_inner.sync().expect("init server failed");
	let server = ServerTop {
		inner: Arc::new(Mutex::new(server_inner))
	};

	let router = Router::new().push(
		Router::with_path("server").get(hello).push(Router::with_path("<request>").post(server))
	).push(
		Router::with_path("<**path>").get(
			StaticDir::new(["admin", 
				"book/book", 
				"data/info/update/nightly",
				"data/info/update/stable",
				"data/info/notice",
				"data/source/chart"]).defaults("index.html").auto_list(true),
		)
	);
	let acceptor = TcpListener::new("127.0.0.1:7878").bind().await;
	Server::new(acceptor).serve(router).await;
}