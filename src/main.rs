use std::env;
use serenity::{
    async_trait,
    client::{
        Client,
        Context,
        EventHandler,
    },
    model::{
        channel::Message,
        gateway::Ready,
    },
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group,
        }
    }
};

#[group]
#[commands(ping)]
struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{:?} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$")) // set bot prefix
        .group(&GENERAL_GROUP);

    // login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start_shards(1).await {
        println!("An error occurred while running the client {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "yes!").await?;

    Ok(())
}
