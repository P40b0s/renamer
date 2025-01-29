use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};


pub fn progressbar(len: u64) -> ProgressBar
{
    let sty = ProgressStyle::with_template(
        "{spinner:.blue} [{elapsed_precise}] {prefix} {bar:40.green/red} {pos:>0}/{len:0} {msg}",
    )
    .unwrap()
    .tick_strings(&[
        "▹▹▹▹▹",
        "▸▹▹▹▹",
        "▹▸▹▹▹",
        "▹▹▸▹▹",
        "▹▹▹▸▹",
        "▹▹▹▹▸",
        "▪▪▪▪▪",
    ])
    .progress_chars("##-");
    let pb = ProgressBar::new(len);
    pb.set_style(sty.clone());
    pb.enable_steady_tick(Duration::from_millis(120));
    pb
}

