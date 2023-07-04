use mgl_core::{
    core::{folder::MinecraftLocation, task::EventListeners},
    installer::install,
};

#[tokio::main]
async fn main() {
    install(
        "1.19.4",
        MinecraftLocation::new("test"),
        EventListeners::new().on_progress(Box::new(|a, b, c| {
            println!("progress: {a}/{b}  step: {c}");
        })),
    )
    .await;
}
