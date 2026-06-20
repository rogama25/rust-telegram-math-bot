use dotenvy::dotenv;
use telers::enums::UpdateType;
use telers::event::telegram;
use telers::event::telegram::Handler;
use telers::methods::SendRichMessage;
use telers::types::{InputRichMessage, Message};
use telers::{Bot, Dispatcher, Router};

#[tokio::main]
async fn main() {
    let _ = dotenv();

    let bot = Bot::from_env();

    let router =
        Router::new("main").on_message(|observer| observer.register(Handler::new(echo_handler)));

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => println!("Bot stopped"),
        Err(err) => println!("Bot stopped, error: {}", err),
    }
}

async fn echo_handler(bot: Bot, message: Message) -> telegram::HandlerResult<()> {
    bot.send(SendRichMessage::new(
        message.chat().id(),
        InputRichMessage::new().markdown(format!("```math\n\n{}\n```", message.text().unwrap())),
    )).await?;
    Ok(())
}
