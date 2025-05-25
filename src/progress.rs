use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress_bar(length: u64) -> ProgressBar {
    let pb = if length > 0 {
        ProgressBar::new(length)
    } else {
        // For unknown length, show an indeterminate bar style
        ProgressBar::new_spinner()
    };

    let style = if length > 0 {
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"
        )
        .unwrap()
        .progress_chars("=> ")
    } else {
        // Indeterminate moving bar
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes} (unknown size)"
        )
        .unwrap()
        .progress_chars("#>-")
    };

    pb.set_style(style);
    pb
}