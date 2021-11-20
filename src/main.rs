#[macro_use]
extern crate tracing;

use poise::serenity_prelude as serenity;
use rosu_v2::Osu;
use tracing_subscriber::FmtSubscriber;
use crate::apis::nucleoid::NucleoidClient;
use crate::apis::open_notify::OpenNotifyClient;

use crate::config::StarlightConfig;

mod commands;
pub mod checks;
pub mod config;
pub mod apis;
pub mod util;

pub struct Data {
    pub config: StarlightConfig,
    pub osu_client: Osu,
    pub open_notify_client: OpenNotifyClient,
    pub nucleoid_client: NucleoidClient,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: Error, ctx: poise::ErrorContext<'_, Data, Error>) {
    match ctx {
        poise::ErrorContext::Setup => panic!("Failed to start bot: {:?}", error),
        poise::ErrorContext::Command(ctx) => {
            println!("Error in command `{}`: {:?}", ctx.command().name(), error);
            let _ = ctx.ctx().say(":warning: Sorry, an error occurred handling the command!").await;
        }
        _ => println!("Other error: {:?}", error),
    }
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter("debug,h2=info,hyper=info")
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set default subscriber");

    info!("Hello, world!");

    let config = config::load();

    let mut options = poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("s?".into()),
            ..Default::default()
        },
        on_error: |err, ctx| Box::pin(on_error(err, ctx)),
        ..Default::default()
    };

    // Fun
    options.command(commands::fun::iss(), |f| f.category("Fun"));

    // Nucleoid
    options.command(commands::nucleoid::nucleoid(), |f| f.category("Nucleoid")
        .subcommand(commands::nucleoid::nucleoid_status(), |f| f.category("Nucleoid"))
        .subcommand(commands::nucleoid::nucleoid_recent_games(), |f| f.category("Nucleoid")));

    // osu!
    options.command(commands::osu::osu_stats(), |f| f.category("osu!")
        .subcommand(commands::osu::osu_stats_catch(), |f| f)
        .subcommand(commands::osu::osu_stats_mania(), |f| f)
        .subcommand(commands::osu::osu_stats_standard(), |f| f)
        .subcommand(commands::osu::osu_stats_taiko(), |f| f));

    // Users
    options.command(commands::users::age(), |f| f.category("Users"));

    // Utils
    options.command(commands::util::ping(), |f| f.category("Util"));
    options.command(commands::util::register_commands(), |f| f.category("Util"));
    options.command(commands::util::about(), |f| f.category("Util"));

    let osu_client = Osu::new(config.osu.client_id, &config.osu.client_secret.clone())
        .await.expect("failed to set up osu! client");
    let open_notify_client = OpenNotifyClient::default();
    let nucleoid_client = NucleoidClient::default();

    poise::Framework::<Data, Error>::build()
        .token(&config.discord.token)
        .user_data_setup(move |ctx, _ready, _framework| Box::pin(async move {
            ctx.set_activity(serenity::Activity::listening("non-robots")).await;
            Ok(Data {
                config: config.clone(),
                osu_client,
                open_notify_client,
                nucleoid_client,
            })
        }))
        .options(options)
        .run()
        .await.unwrap();
}
