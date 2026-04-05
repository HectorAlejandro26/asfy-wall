use crate::cache::{Cache, CacheManager};
use crate::config::OrderBy;
use anyhow::{Context, Ok, Result, anyhow, bail};
use rand::seq::SliceRandom;
use std::path::PathBuf;
use std::process::Command;

pub fn list_images(dir: &PathBuf) -> Result<Vec<String>> {
    let dir = dir
        .canonicalize()
        .with_context(|| format!("Failed to resolve path: {:?}", dir))?;
    if !dir.is_dir() {
        bail!("Path is not a directory: {:?}", dir.display());
    }

    let images = dir
        .read_dir()
        .context("Failed to read directory")?
        .filter_map(Result::ok)
        .filter(|entry| {
            let path = entry.path();
            path.is_file()
                && path.extension().is_some_and(|ext| {
                    matches!(
                        ext.to_str().unwrap_or("").to_ascii_lowercase().as_str(),
                        "jpg" | "jpeg" | "png" | "gif" | "webp"
                    )
                })
        })
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();

    Ok(images)
}

pub struct WallyEngine {
    images_dir: PathBuf,
    order_by: OrderBy,
    reverse: bool,
    external_args: Vec<String>,
    cache: Cache,
    cache_manager: CacheManager,
    dry_run: bool,
}

impl WallyEngine {
    pub fn new(
        images_dir: PathBuf,
        order_by: OrderBy,
        reverse: bool,
        external_args: Vec<String>,
        cache: Cache,
        cache_manager: CacheManager,
        dry_run: bool,
    ) -> Result<Self> {
        let images_dir = images_dir
            .canonicalize()
            .with_context(|| format!("Failed to resolve path: {:?}", images_dir))?;

        if !images_dir.is_dir() {
            bail!(
                "Provided path is not an image directory: {:?}",
                images_dir.display()
            );
        }

        Ok(Self {
            images_dir,
            order_by,
            reverse,
            external_args,
            cache,
            cache_manager,
            dry_run,
        })
    }

    pub fn run(&mut self, reorder: bool, set_index: Option<usize>) -> Result<()> {
        if self.cache.images.is_empty() {
            bail!("Directory has no images to use");
        }

        if reorder {
            match self.order_by {
                OrderBy::None => {
                    let mut rng = rand::rng();
                    self.cache.images.shuffle(&mut rng);
                }
                OrderBy::Name => self.cache.images.sort(),
                OrderBy::CreatedAt | OrderBy::ModifiedAt => {
                    self.cache.images.sort_by(|a, b| {
                        let path_a = self.images_dir.join(a);
                        let path_b = self.images_dir.join(b);

                        let meta_a = std::fs::metadata(&path_a);
                        let meta_b = std::fs::metadata(&path_b);

                        let time_a = meta_a
                            .and_then(|m| {
                                if matches!(self.order_by, OrderBy::CreatedAt) {
                                    m.created()
                                } else {
                                    m.modified()
                                }
                            })
                            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

                        let time_b = meta_b
                            .and_then(|m| {
                                if matches!(self.order_by, OrderBy::CreatedAt) {
                                    m.created()
                                } else {
                                    m.modified()
                                }
                            })
                            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

                        time_a.cmp(&time_b)
                    });
                }
            }

            if self.reverse {
                self.cache.images.reverse();
            }

            self.cache.index_now = 0;
        }

        if let Some(idx) = set_index {
            self.cache.index_now = idx;
        }

        // Control de daños: por si el índice se desfasó mágicamente
        if self.cache.index_now >= self.cache.images.len() {
            self.cache.index_now = 0;
        }

        // 1. Apuntamos a la imagen actual
        let current_image = &self.cache.images[self.cache.index_now];
        let image_path = self.images_dir.join(current_image);

        // 2. Ejecutar comando
        self.execute_cmd(image_path)?;

        // 3. Avanzamos el tambor (vuelve a 0 si llega al final)
        self.cache.index_now = (self.cache.index_now + 1) % self.cache.images.len();

        // 4. Guardamos el estado para que la próxima ejecución sepa dónde seguir
        self.cache_manager.write(&self.cache)?;

        Ok(())
    }

    pub fn execute_cmd(&self, image_path: PathBuf) -> Result<()> {
        if !self.dry_run {
            let status = Command::new("awww")
                .arg("img")
                .arg(image_path.display().to_string())
                .args(&self.external_args)
                .status()
                .with_context(|| anyhow!("Failed to execute 'awww' or command not found"))?;

            if !status.success() {
                bail!("Command 'awww' failed with exit status: {}", status);
            }
        } else {
            println!(
                "Dry run would execute:\nawww img {} {}",
                image_path.display(),
                self.external_args.join(" ")
            );
        }

        Ok(())
    }
}
