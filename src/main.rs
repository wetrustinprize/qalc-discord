use poise::serenity_prelude as serenity;
use std::time::Duration;
use tokio::process::Command;
use tokio_process_tools::*;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Calculates equations using libqalculate
///
/// Examples:
/// /calc 2 + 2
/// /calc 2days hours * 3
/// /calc 20hours to seconds
#[poise::command(slash_command)]
async fn calc(ctx: Context<'_>, #[description = "Formula"] formula: String) -> Result<(), Error> {
    println!("Received a request from {} to calculate `{}`", ctx.author().name, formula);

    let mut cmd = Command::new("qalc");
    cmd.arg(&formula);

    let mut proc = Process::new(cmd).spawn_single_subscriber().unwrap();

    match ctx.defer().await {
        Ok(_) => {},
        Err(e) => {
            println!("An error occoured when defering the context: {}", e);
            return Ok(());
        }
    }

    let result = proc
        .wait_for_completion_with_output_or_terminate(
            Duration::from_secs(2),
            Duration::from_secs(1),
            Duration::from_secs(1),
            LineParsingOptions::default(),
        )
        .await;

    match result {
        Ok(output) => {
            if output.status.success() {
                let message = output.stdout.join("");
                ctx.say(message).await?;
            } else {
                println!("Failed to wait for qalc result: `{}`, took too long.", formula);
                ctx.say("This operation took too long! We canceled the calculation. ;)").await?;
            }

            Ok(())
        }
        Err(err) => {
            println!("Failed to wait for qalc result: `{}`: {}", formula, err);
            ctx.say("There was an internal error while calculating the result, please try again later").await?;
            Ok(())
        }
    }
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
