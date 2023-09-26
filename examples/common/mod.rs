// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::path::PathBuf;

use clap::crate_version;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "The TiKV Project Authors")]
#[command(version = crate_version!())]
pub struct CommandArgs {
    #[arg(long, default_value = "localhost:2379")]
    pub pd: Vec<String>,

    #[arg(long)]
    pub ca: Option<PathBuf>,

    #[arg(long)]
    pub cert: Option<PathBuf>,

    #[arg(long)]
    pub key: Option<PathBuf>,
}
pub fn parse_args() -> CommandArgs {
    CommandArgs::parse()
}
