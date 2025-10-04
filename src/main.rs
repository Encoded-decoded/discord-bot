use poise::{serenity_prelude::all::{ChannelId, CreateMessage, GuildId}};
use dotenvy::dotenv;
use poise::serenity_prelude as serenity;

#[derive(Debug)]
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let guild_id = std::env::var("GUILD_ID")
        .expect("missing GUILD_ID")
        .parse::<GuildId>()
        .expect("GUILD_ID must be a valid u64");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(async move { event_handler(ctx, event, framework, data).await })
            },
            manual_cooldowns: true,
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                // Register commands in the specified guild
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id)
                    .await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    client.start().await.expect("Error starting client");
}

async fn event_handler(
    ctx: &serenity::Context, // Change this line
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        poise::serenity_prelude::FullEvent::GuildMemberAddition { new_member } => {
            let _greet_channel = ChannelId::new(1392444888586518540)
                .send_message(
                    &ctx.http, // Change this line
                    CreateMessage::new().content(format!(
                        "Welcome, {new_member}! It’s a privilege to have you around, it’s not the same without you, trust me.",
                    )),
                )
                .await?;
            log_eventhandler(&format!("Welcomed new_user: {new_member} in #welcome"), ctx)
                .await
                .expect("Error while logging Welcome message");
            println!("Greeted {}", new_member);
        }
        _ => {}
    }
    Ok(())
}

async fn log_eventhandler(input: &str, ctx: &serenity::Context) -> Result<(), Box<Error>> {
    use serenity::prelude::CacheHttp;
    match ChannelId::new(1392850628409163956)
        .send_message(
            ctx.http(),
            CreateMessage::new().content(format!("LOG: {}", input)),
        )
        .await
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}")
        }
    };

    Ok(())
}

#[poise::command(slash_command)]
/// Reply with pong
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Pong!").await?;
    log_eventhandler("/ping", ctx.serenity_context()).await.expect("Error logging");
    Ok(())
}
