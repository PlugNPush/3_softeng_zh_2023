use std::{net::TcpListener, sync::Arc};

use axum::{http::Request, Router};
use futures::StreamExt;
use models::{Notification, TemperatureMeasurement};
use server::{router::api_router, state::AppState};
use tokio_tungstenite::tungstenite;

#[tokio::test]
async fn new_measurement_causes_notification() {
    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    let server_addr = listener.local_addr().unwrap();

    let state = Arc::new(AppState::default());
    let router = Router::new().nest("/api", api_router(state));

    // spawn an actual API server on a random port
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });

    let client = hyper::Client::new();

    // open a websocket connection to the server
    let (mut socket, _response) =
        tokio_tungstenite::connect_async(format!("ws://{server_addr}/api/notifications"))
            .await
            .unwrap();

    // create a new measurement via HTTP
    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{server_addr}/api/measurements/random"))
                .method("POST")
                .body(hyper::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let new_measurement = serde_json::from_slice::<TemperatureMeasurement>(&body).unwrap();

    // receive the notification via websocket
    let msg = match socket.next().await.unwrap().unwrap() {
        tungstenite::Message::Text(msg) => msg,
        other => panic!("expected a text message but got {other:?}"),
    };
    let notification: Notification = serde_json::from_str(&msg).unwrap();
    let notif_measurement = match notification {
        Notification::New(m) => m,
        _ => panic!("expected a new measurement notification"),
    };

    // check that the notification contains the same new measurement
    // as the HTTP response
    assert_eq!(new_measurement, notif_measurement);
}
