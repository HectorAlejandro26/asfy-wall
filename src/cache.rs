use crate::APP_NAME;
use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cache {
    pub images_dir: PathBuf,
    pub index_now: usize,
    pub images: Vec<String>,
}

pub struct CacheManager {
    cache_file: PathBuf,
    images_dir: PathBuf,
    images: Vec<String>,
}

impl CacheManager {
    pub fn new(images_dir: PathBuf, images: Vec<String>) -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join(APP_NAME);

        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)
                .with_context(|| format!("Failed to create cache directory: {:?}", cache_dir))?;
        }

        let cache_file = cache_dir.join("status.toml");

        Ok(Self {
            cache_file,
            images_dir,
            images,
        })
    }

    pub fn load(&self) -> Result<(Cache, bool)> {
        if !self.cache_file.exists() {
            let default_cache = Cache {
                images_dir: self.images_dir.clone(),
                index_now: 0,
                images: self.images.clone(),
            };
            self.write(&default_cache)?;
            return Ok((default_cache, true));
        }

        let content = fs::read_to_string(&self.cache_file)
            .with_context(|| format!("Failed to read cache at {:?}", self.cache_file))?;

        let mut cache: Cache = toml::from_str(&content)
            .with_context(|| "Corrupted status.toml file. Did someone tinker with it manually?")?;

        let mut is_dirty = false;

        // Clonamos y ordenamos para comparar contenido real, no el orden
        let mut cached_sorted = cache.images.clone();
        cached_sorted.sort();
        let mut fresh_sorted = self.images.clone();
        fresh_sorted.sort();

        if cache.images_dir != self.images_dir || cached_sorted != fresh_sorted {
            cache.images_dir = self.images_dir.clone();
            cache.images = self.images.clone();
            cache.index_now = 0;
            self.write(&cache)?;
            is_dirty = true;
        }

        Ok((cache, is_dirty))
    }

    pub fn write(&self, cache: &Cache) -> Result<()> {
        let toml_str =
            toml::to_string_pretty(&cache).with_context(|| "Failed to serialize cache")?;
        fs::write(&self.cache_file, toml_str)
            .with_context(|| format!("Failed to write cache to {:?}", self.cache_file))?;

        Ok(())
    }
}
