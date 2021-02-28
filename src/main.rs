#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate prometheus;
extern crate pty;
extern crate stderrlog;

mod config;
mod middlewares;
mod routes;

use config::{Server,Config};
use handlebars::Handlebars;
use std::sync::Arc;
use std::{io::Result, path::PathBuf};
use structopt::StructOpt;
use warp::Filter;

/// WeTTy server
#[derive(Debug, StructOpt)]
struct Args {
    /// Configuration file config path
    #[structopt(short, long, parse(from_os_str), default_value = "configs/config.toml")]
    config: PathBuf,
    /// Silence all output
    #[structopt(short, long)]
    quiet: bool,
    /// Increase message verbosity
    #[structopt(short, long, parse(from_occurrences))]
    verbose: usize,
    /// Print default config
    #[structopt(short, long)]
    print: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    if args.print {
        return Ok(Config::print_default());
    }

    stderrlog::new()
        .module(module_path!())
        .quiet(args.quiet)
        .verbosity(args.verbose)
        .timestamp(stderrlog::Timestamp::Second)
        .init()
        .unwrap();

    let log = warp::log("wetty");
    let metrics = warp::log::custom(middlewares::metrics);

    let conf = Config::from_file(&args.config)?;
    info!("config loaded; path={:?}", args.config);

    let metrics_route = warp::path!("metrics").and_then(routes::metrics::handler).with(metrics);
    let health = warp::path!("health").and_then(routes::health::handler).with(metrics);

    let base = warp::path(conf.server.base);
    let socket = base.clone().and(warp::path!("socket.io"))
        .and(warp::ws())
        .and_then(routes::socket::handler)
        .with(metrics);

    let hb = Arc::new(Handlebars::new());
    let client = base.clone().or(base.clone().and(warp::path!("ssh" / String )))
        .and(||warp::any().map(|| conf.server.clone()))
        .map(move |_user, config: Server|routes::html::render(hb.clone(), config.title,config.base))
        .with(metrics);

    let routes = warp::fs::dir("client/build")
        .or(client)
        .or(metrics_route)
        .or(health)
        .or(socket)
        .with(log);

    info!(
        "Server started; address={}",
        conf.server.address,
    );
    warp::serve(routes).run(conf.server.address).await;
    info!("Server shutting down");
    Ok(())
}
