use warp::Filter;
use futures_util::FutureExt;
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let ws_path = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| {
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    let static_path = warp::fs::dir("assets");

    let game_file = warp::path("game.toml")
        .and(warp::fs::file("game.toml"));

    warp::serve(ws_path.or(game_file.or(static_path)))
        .run(([0, 0, 0, 0], 8080))
        .await;
}