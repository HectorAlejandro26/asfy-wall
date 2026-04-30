use crate::config::OrderBy;
use crate::constants::APP_NAME;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = APP_NAME, 
    version, 
    about = "A blazing fast wallpaper manager and execution engine.", 
    long_about = "asfy-wall is a tool that manages, caches, and cycles through your image directories, delegating the heavy lifting to the 'awww' command."
)]
pub struct Args {
    #[arg(long, help = "Force a reorder of the images based on the 'order_by' setting")]
    pub reorder: bool,

    #[arg(
        short,
        long,
        help = "Override the 'order_by' setting from the config",
        long_help = "Temporarily override the sorting method. If this differs from your config, it automatically triggers a reorder."
    )]
    pub order_by: Option<OrderBy>,

    #[arg(
        short, 
        long, 
        num_args = 0..2, 
        default_missing_value = "true", 
        action = clap::ArgAction::Set,
        help = "Reverse the image order (e.g., --reverse or --reverse false)"
    )]
    pub reverse: Option<bool>,

    #[arg(short, long, help = "Override the base directory where images are stored")]
    pub images_dir: Option<String>,

    #[arg(long, help = "Preview what would be executed without actually calling 'awww'")]
    pub dry_run: bool,

    #[arg(long, help = "Jump directly to a specific image index in the cache")]
    pub set_index: Option<usize>,

    #[arg(last = true, help = "Arguments to pass directly to the external command (awww)")]
    pub external_args: Vec<String>,
}
