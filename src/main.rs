mod app;
mod args;
mod cache;
mod config;
mod constants;
mod engine;

use anyhow::Result;

fn main() -> Result<()> {
    app::AsfyWallApp::run()
}
