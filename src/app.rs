use crate::args::Args;
use crate::cache::CacheManager;
use crate::config::ConfigManager;
use crate::engine::WallEngine;
use anyhow::{Context, Result, anyhow};
use clap::Parser;
use std::path::PathBuf;

pub struct AsfyWallApp;

impl AsfyWallApp {
    pub fn run() -> Result<()> {
        let args = Args::parse();

        let config_manager = ConfigManager::new()?;
        let config = config_manager.load()?;

        // Resolución de la carpeta de imágenes (Args > Config)
        let images_dir = args
            .images_dir
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .or_else(|| {
                let p = &config.images_dir;
                if !p.as_os_str().is_empty() {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("Error: No images directory provided"))?;

        let images_dir_str = images_dir.to_string_lossy();
        let expanded_dir = shellexpand::full(&images_dir_str)
            .with_context(|| anyhow!("Failed to expand path: {}", images_dir_str))?
            .into_owned();
        let images_dir = PathBuf::from(expanded_dir);

        let order_by = args.order_by.unwrap_or(config.order_by);
        let reverse = args.reverse.unwrap_or(config.reverse);
        let external_args = if args.external_args.is_empty() {
            config.external_args
        } else {
            args.external_args
        };

        let images_list = WallEngine::scan_directory(&images_dir)?;

        let cache_manager = CacheManager::new(images_dir.clone(), images_list)?;
        let (current_cache, cache_changed) = cache_manager.load()?;

        let reorder = cache_changed || args.reorder;

        let mut engine = WallEngine::new(
            images_dir,
            order_by,
            reverse,
            external_args,
            current_cache,
            cache_manager,
            args.dry_run,
        )?;

        engine.execute(reorder, args.set_index)?;

        Ok(())
    }
}
