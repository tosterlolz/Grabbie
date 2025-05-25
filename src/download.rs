use anyhow::{Context, Result};
use colored::*;
use indicatif::ProgressBar;
use reqwest::{Client, Response};
use tokio::fs::File;
use tokio::io::{AsyncReadExt};
use futures_util::StreamExt;
use std::path::Path;

pub async fn download_get(client: &Client, url: &str, pb: ProgressBar) -> Result<()> {
    let response = client.get(url).send().await.context("request failed")?;
    download_response(response, url, pb).await
}

pub async fn download_post(client: &Client, url: &str, pb: ProgressBar, data_file: Option<&str>) -> Result<()> {
    let body = match data_file {
        Some(path) => {
            let mut file = File::open(path).await.context("cannot open data file")?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).await.context("cannot read data file")?;
            contents
        }
        None => Vec::new(),
    };

    let response = client.post(url).body(body).send().await.context("request failed")?;
    download_response(response, url, pb).await
}

async fn download_response(response: Response, url: &str, pb: ProgressBar) -> Result<()> {
    let total_size = response
        .content_length()
        .unwrap_or(0);

    pb.set_length(total_size);

    let filename = url
        .split('/')
        .last()
        .filter(|s| !s.is_empty())
        .unwrap_or("download.bin");

    let path = Path::new(filename);
    let mut file = tokio::fs::File::create(path).await.context("failed to create file")?;

    let mut stream = response.bytes_stream();

    let mut downloaded: u64 = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.context("error while downloading chunk")?;
        tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await.context("error writing to file")?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message(format!("{} {}", "✔ Downloaded:".green(), filename));
    Ok(())
}
