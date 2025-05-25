use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress_bar(length: u64) -> ProgressBar {
    let pb = ProgressBar::new(length);
    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"
    ).unwrap()
    .progress_chars("=> ");

    pb.set_style(style);
    pb
}
