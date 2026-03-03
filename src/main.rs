use poise::serenity_prelude as serenity;
use qalculate_rs::Qalculate;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn calc(
    ctx: Context<'_>,
    #[description = "Formula"] formula: String,
) -> Result<(), Error> {
    let formula_clone = formula.clone();
    let response = tokio::task::spawn_blocking(move || {
        let calc = Qalculate::new().unwrap();
        calc.calculate_string(&formula_clone).unwrap()
    })
    .await?;

    ctx.say(format!("`{}` = **{}**", formula, response)).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = dotenv::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::DIRECT_MESSAGES;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![calc()],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            println!("Discord {} is ready!", ready.user.name);

            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
