#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

extern crate core;

mod event;
mod response;
mod routes;
mod service;
mod app;
mod logger;
mod types;
use anyhow::{Result};
use log::{error};
use crate::event::Evt;


#[tokio::main]
async fn main() -> Result<()> {
    app::make_sure_single_case();
    match app::run().await {
        Ok(_) => {}
        Err(err) => {
            error!("app start error,{}",err);
        }
    }
    Ok(())
}

