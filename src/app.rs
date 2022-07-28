use std::{env, fs};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use std::sync::Arc;
use anyhow::{anyhow, Result};
use crate::{event, Evt};
use log::{error};
use rusqlite::Connection;
use sciter::{Element, make_args, Value};
#[cfg(target_os = "windows")]
use std::ptr::null;
use std::thread::{sleep, spawn};
use std::time::Duration;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{GetLastError, WIN32_ERROR},
    Globalization::GetUserDefaultUILanguage,
    System::Threading::{CreateMutexW, OpenMutexW},
    UI::WindowsAndMessaging::{MB_OK, MessageBoxW},
};
use crate::service::Service;

pub async fn run() -> Result<()> {
    // load resource
    let resource = include_bytes!("resource.rc");

    // set script options
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8        // Enables `Sciter.machineName()`.  Required for opening file dialog (`view.selectFile()`)
            | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8    // Enables opening file dialog (`view.selectFile()`)
    )).map_err(|_| anyhow!("unknown error"))?;
    //set debug module
    #[cfg(debug_assertions)]
    sciter::set_options(sciter::RuntimeOptions::DebugMode(true))
        .map_err(|_| anyhow!("unknown error"))?;
    // init backend window
    let mut window = sciter::WindowBuilder::main_window().with_size((0, 0)).create();
    window.archive_handler(resource).map_err(|_| anyhow!("unknown error!"))?;
    window.load_file("this://app/index.html");

    let app_data_path = window.get_host().call_function("app_data_path", &make_args!(""))
        .map_err(|e| anyhow!("{}",e))?.as_string();
    // init app data dir
    app_data_dir(app_data_path);
    // init logger
    crate::logger::init()?;
    // init sqlite
    sqlite_init().await?;
    // crate a channel
    let (body_sender, body_receiver) = flume::unbounded();
    // create a service
    let service = Arc::new(Mutex::new(Service::new()?));
    window.event_handler(Evt {
        body_sender: body_sender.clone(),
        service: service.clone(),
    });
    listen_frpc_logs(service.clone());
    // listen request service
    event::do_request_services(body_receiver);
    window.run_app();
    service.lock().exit()?;
    Ok(())
}

pub fn listen_frpc_logs(service: Arc<Mutex<Service>>) {
    let service_stdout = service.clone();
    spawn(move || -> Result<()> {
        loop {
            let stdout = service_stdout.lock().proc.stdout.take();
            let element = Element::create("html")?;
            match stdout {
                None => {
                    sleep(Duration::from_secs(1))
                }
                Some(stdout) => {
                    let mut child_out = BufReader::new(stdout);
                    loop {
                        let mut line = String::new();
                        child_out.read_line(&mut line).unwrap();
                        if line == "" {
                            break;
                        }
                        element.broadcast_event("log", true, Some(Value::from(&line)))?;
                    }
                }
            };
        }
    });
    spawn(move || -> Result<()> {
        loop {
            let stdout = service.lock().proc.stderr.take();
            let element = Element::create("html")?;
            match stdout {
                None => {
                    sleep(Duration::from_secs(1))
                }
                Some(stdout) => {
                    let mut child_out = BufReader::new(stdout);
                    loop {
                        let mut line = String::new();
                        child_out.read_line(&mut line).unwrap();
                        if line == "" {
                            break;
                        }
                        element.broadcast_event("log", true, Some(Value::from(&line)))?;
                    }
                }
            };
        }
    });
}

pub type AppDataPath = PathBuf;

pub fn app_data_dir(path: Option<String>) -> &'static AppDataPath {
    static INSTANCE: OnceCell<AppDataPath> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        match path {
            Some(str) => PathBuf::from(str).join("frper"),
            None => match env::current_exe() {
                Ok(e) => match e.parent() {
                    Some(path) => path.to_path_buf(),
                    None => {
                        error!("get current_exe directory failed");
                        panic!("get current exe error")
                    }
                }
                Err(err) => {
                    error!("get current_exe directory failed {}",err);
                    panic!("get current exe error")
                }
            }
        }
    })
}

pub async fn get_db_conn() -> Result<Connection> {
    get_db_conn_sync()
}

pub fn get_db_conn_sync() -> Result<Connection> {
    let app_data_dir = PathBuf::from(app_data_dir(None));
    if !app_data_dir.exists() {
        fs::DirBuilder::new().recursive(true).create(&app_data_dir)?;
    }
    let path = app_data_dir.join("data.db");
    let conn = Connection::open(path)?;
    Ok(conn)
}

pub async fn sqlite_init() -> Result<()> {
    let conn = get_db_conn().await?;
    let sql = r#"
    create table if not exists client_setting(
                    key                 text primary key not null,
                    default_value       text,
                    value               text not null,
                    desc                text not null,
                    options             text,
                    remark              text,
                    set_type            text not null default 'common'
                )"#;
    conn.execute(sql, [])?;
    let sql = "select key from client_setting where key=?";
    if let Err(_) = conn.query_row(sql, ["server_addr"], |row| row.get::<&str, String>("key")) {
        let sql = include_str!("../sql/client_setting.sql");
        conn.execute_batch(sql)?;
    }
    conn.execute(r#"CREATE TABLE if not exists "proxy_setting" (
  "id" text NOT NULL,
  "name" TEXT NOT NULL,
  "base" TEXT NOT NULL,
  "local" TEXT NOT NULL,
  "slb" TEXT NOT NULL,
  "proxy_type" TEXT NOT NULL,
  "proxy_type_value" TEXT NOT NULL,
  "enabled" TEXT NOT NULL,
  PRIMARY KEY ("id")
);"#, [])?;
    Ok(())
}

/// make sure app running at single-case
#[cfg(target_os = "windows")]
pub fn make_sure_single_case() {
    unsafe {
        let _ = OpenMutexW(0, true, "frper.xsa.link@fuyoo");
        let WIN32_ERROR(code) = GetLastError();
        if code == 2 {
            // generate lock
            let _handle = CreateMutexW(null(), true, "frper.xsa.link@fuyoo");
            // if impl update service should be unlock
            // can be use ReleaseMutex(_handle) api https://docs.microsoft.com/en-us/previous-versions/ms942538(v=msdn.10)
        } else {
            // get local language
            let lang = GetUserDefaultUILanguage();
            // Adapt to Chinese and english
            if lang == 2052u16 {
                MessageBoxW(None, "Frper 正在运行中, 请不要重复运行！", "提示", MB_OK);
            } else {
                MessageBoxW(
                    None,
                    "Frper is Running, Please do not run it again!",
                    "Tips",
                    MB_OK,
                );
            }
            std::process::exit(0);
        }
    }
}

/// make sure app running at single-case
#[cfg(not(target_os = "windows"))]
pub fn make_sure_single_case() {
    //todo: unix system should be complete
}


