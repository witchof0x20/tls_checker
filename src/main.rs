// Copyright 2022 Jade
// This file is part of tls_checker.
//
// tls_checker is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License version 3 as published by the Free Software Foundation
//
// tls_checker is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with tls_checker. If not, see <https://www.gnu.org/licenses/>.
use clap::Parser;
use futures::{stream, StreamExt};
use reqwest::Client;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

/// Simple program to filter out a list of websites down to
/// just the ones where HTTPS connects successfully
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the top website csv (number, hostname)
    csv_path: PathBuf,
    /// Path to write a website list to
    out_path: PathBuf,
    /// Size of the desired list. If absent, the program will go through all urls
    #[clap(short, long)]
    count: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse cli args
    let args = Args::parse();
    // Store the list of websites
    let mut websites = File::open(args.csv_path)
        .map(BufReader::new)?
        .lines()
        .map(|line| {
            line.and_then(|line| {
                let mut line = line.split(',');
                let rank: usize = line
                    .next()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Rank missing"))?
                    .parse()
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                let host = line
                    .next()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Host missing"))?;
                Ok((rank, host.to_owned()))
            })
        })
        .collect::<Result<Vec<_>, io::Error>>()?;
    // Sort by rank just in case
    websites.sort_unstable_by_key(|&(rank, _)| rank);
    // We either want up to every website, or only a limited subset number of sites
    let num_to_generate = args.count.unwrap_or(websites.len());
    // Try requesting each website over https
    let client = Client::builder().https_only(true).build()?;
    let https_hosts: Vec<String> = stream::iter(websites)
        .map(|(rank, host)| {
            let client = &client;
            async move {
                println!("Navigating to {host} {rank}/{num_to_generate}");
                (
                    host.clone(),
                    client.get(format!("https://{host}")).send().await,
                )
            }
        })
        .buffer_unordered(10)
        .filter_map(|(host, resp)| async move {
            match resp {
                Ok(_) => Some(host),
                Err(err) => {
                    eprintln!("{host} does not support https or otherwise failed to connect with error {}", err);
                    None
                }
            }
        })
        .take(num_to_generate)
        .collect::<Vec<String>>()
        .await;
    // Output to a file
    let mut out_file = File::create(args.out_path).map(BufWriter::new)?;
    for (rank, host) in https_hosts.into_iter().enumerate() {
        writeln!(out_file, "{rank},{host}")?;
    }

    Ok(())
}
