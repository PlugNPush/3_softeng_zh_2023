use axum::{
    extract::{
        ws::{Message::Text, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};

use crate::state::AppState;

pub async fn subscribe(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let mut notification_receiver = state.subscribe();
    while let Ok(notification) = notification_receiver.recv().await {
        if socket
            .send(Text(serde_json::to_string_pretty(&notification).unwrap()))
            .await
            .is_err()
        {
            // client disconnected
            return;
        }
    }
}
