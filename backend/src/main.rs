use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(game_server)
}

async fn game_server(mut ws: WebSocket) {
    while let Some(Ok(msg)) = ws.recv().await {
        if let Message::Text(msg) = msg {
            if ws
                .send(Message::Text(format!("You said: {msg}")))
                .await
                .is_err()
            {
                break;
            }
        }
    }
}
