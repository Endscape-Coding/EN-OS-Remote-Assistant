use teloxide::prelude::*; 
pub async fn message(bot: Bot, msg: Message) -> Result<(),teloxide::RequestError> {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, text).await?;
            }
        None => {
            bot.send_message(msg.chat.id, "Hi").await?;
            }
    }
    Ok(())
}
