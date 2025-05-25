mod cli;
mod download;
mod error;

use clap::Parser;
use error::{GrabbieError, Result};
use indicatif::{MultiProgress, ProgressBar};
use std::sync::Arc;
use tokio::task;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    let mp = Arc::new(MultiProgress::new());
    let client = reqwest::Client::new();

    let post_mode = args.post;
    let urls = args.urls;
    let data_file = args.data_file.as_deref();

    let mut handles = vec![];

    for url in urls {
        let client = client.clone();
        let mp_clone = mp.clone();
        let url_clone = url.clone();
        let data_file = data_file.map(str::to_string);

        let handle = task::spawn(async move {
            let pb = mp_clone.add(ProgressBar::new_spinner());
            pb.set_message(format!("Downloading {}", url_clone));

            let res = if post_mode {
                download::download_post(&client, &url_clone, pb.clone(), data_file.as_deref()).await
            } else {
                download::download_get(&client, &url_clone, pb.clone()).await
            };

            match res {
                Ok(_) => {
                    pb.finish_with_message(format!("Done: {}", url_clone));
                    Ok(())
                }
                Err(e) => {
                    pb.finish_with_message(format!("Error {}: {}", url_clone, e));
                    Err(e)
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let inner_res = handle.await.map_err(|e| GrabbieError::Other(format!("Join error: {}", e)))?;
        inner_res?;
    }

    let _ = mp.clear();

    Ok(())
}