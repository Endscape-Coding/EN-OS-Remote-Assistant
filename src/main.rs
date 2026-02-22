use teloxide::prelude::*;

mod handlers;

#[tokio::main]
async fn main() {
    println!("EN-OS Remote Assistant creator");
    pretty_env_logger::init();
    log::info!("Starting bot");
    let bot = Bot::new("7776624127:AAHjL9T8PJw4myCeqpGaUVZnts1FXWQU5jY");
    teloxide::repl(bot, |bot, msg| {
    async move {
        handlers::message(bot, msg).await?;
        Ok(())
       }
    }).await;
}
