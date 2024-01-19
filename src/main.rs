use salvo::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct User {
	name: String,
	id: i64,
	hashed_code: String
}

#[handler]
async fn server(req: &mut Request) -> String {
	let user = req.param("user").unwrap();
	let user: User = serde_json::from_str(user).unwrap();
	let re = format!("{:#?}", user);
	println!("{}", re);
	re
}

#[handler]
async fn hello() -> &'static str {
	"Hello World"
}

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt().init();

	let router = Router::new().push(
		Router::with_path("server").get(hello).push(Router::with_path("<user>").post(server))
	).push(
		Router::with_path("<**path>").get(
			StaticDir::new(["admin", "play", "book/book"]).defaults("index.html")
		)
	);
	let acceptor = TcpListener::new("localhost:7878").bind().await;
	Server::new(acceptor).serve(router).await;
}