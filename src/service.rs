use std::env;
use std::fs::{DirBuilder, OpenOptions};
use std::io::{Write};
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::path::{PathBuf};
use std::sync::mpsc::{channel};
use std::process::{Child, Command, Stdio};
use std::thread::spawn;
use anyhow::{anyhow, Result};
use log::{error};

use crate::app::{app_data_dir, get_db_conn_sync};
use crate::routes::settings::{Cfg, ProxyTable};
use crate::types::{Base, Http, Https, Local, Slb, Stcp, Sudp, Tcp, TcpMux, Udp, Xtcp};


#[derive(Debug)]
pub struct Service {
    pub proc: Child,
}

impl Service {
    pub fn new() -> Result<Self> {
        let cfg_file = app_data_dir(None).join("frpc.ini").display().to_string();
        let current = env::current_exe()?;
        let mut cmd = Command::new(current
            .parent()
            .expect("frp directory not exits!")
            .join("frpc"));
        #[cfg(target_os = "windows")]
            let proc = cmd.arg("-c")
            .arg(cfg_file)
            .stdout(Stdio::piped())
            .creation_flags(0x08000000)
            .spawn()?;
        #[cfg(not(target_os = "windows"))]
            let proc = cmd.
            arg("-c")
            .arg(cfg_file)
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(Service {
            proc
        })
    }
    pub fn restart(&mut self) -> Result<()> {
        generate_ini_config(None)?;
        match self.proc.kill() {
            Ok(_) => {
                let (s, r) = channel();
                spawn(move || -> Result<()>{
                    let src = Service::new()?;
                    s.send(src.proc)?;
                    Ok(())
                });
                self.proc = r.recv()?;
            }
            Err(err) => {
                let (s, r) = channel();
                spawn(move || -> Result<()>{
                    let src = Service::new()?;
                    s.send(src.proc)?;
                    Ok(())
                });
                self.proc = r.recv()?;
                error!("{}",err)
            }
        }
        Ok(())
    }
    pub fn exit(&mut self) -> Result<()> {
        match self.proc.kill() {
            Ok(_) => {}
            Err(err) => {
                error!("{}",err);
            }
        }
        Ok(())
    }
}

/// generate a ini config
pub fn generate_ini_config(path: Option<String>) -> Result<()> {
    let conn = get_db_conn_sync()?;
    let mut stmt = conn.prepare("select key,value,desc,set_type from client_setting")?;
    let rows = stmt.query_map([], |row| {
        Ok(Cfg {
            key: row.get("key")?,
            default_value: None,
            value: row.get("value")?,
            desc: row.get("desc")?,
            options: None,
            remark: None,
            set_type: row.get("set_type")?,
        })
    })?;
    let mut cfg_vec = vec![];
    cfg_vec.push("[common]".to_string());
    for row in rows {
        let cfg = row?;
        if cfg.value.as_str() != "" {
            cfg_vec.push(format!("{}={}", cfg.key, cfg.value));
        }
    }

    let mut stmt = conn.prepare("select * from proxy_setting where enabled=?1")?;
    let rows = stmt.query_map(["true"], |row| {
        Ok(ProxyTable {
            id: row.get("id")?,
            name: row.get("name")?,
            base: row.get("base")?,
            local: row.get("local")?,
            slb: row.get("slb")?,
            proxy_type: row.get("proxy_type")?,
            proxy_type_value: row.get("proxy_type_value")?,
            enabled: true,
        })
    })?;
    for row in rows {
        let row = row?;
        cfg_vec.push(format!("[{}]", row.name));
        cfg_vec.push(format!("type={}", &row.proxy_type));
        let base: Base = serde_json::from_str(&row.base)?;
        base.insert_to_vec(&mut cfg_vec);
        let local: Local = serde_json::from_str(&row.local)?;
        local.insert_to_vec(&mut cfg_vec);
        let slb: Slb = serde_json::from_str(&row.slb)?;
        slb.insert_to_vec(&mut cfg_vec);
        match row.proxy_type.as_str() {
            "tcp" => {
                let data: Tcp = serde_json::from_str(&row.proxy_type_value)?;
                data.insert_to_vec(&mut cfg_vec);
            }
            "udp" => {
                let data: Udp = serde_json::from_str(&row.proxy_type_value)?;
                data.insert_to_vec(&mut cfg_vec);
            }
            "http" => {
                let data: Http = serde_json::from_str(&row.proxy_type_value)?;
                data.insert_to_vec(&mut cfg_vec);
            }
            "https" => {
                let data: Https = serde_json::from_str(&row.proxy_type_value)?;
                data.insert_to_vec(&mut cfg_vec);
            }
            "stcp" => {
                let data: Stcp = serde_json::from_str(&row.proxy_type_value)?;
                data.insert_to_vec(&mut cfg_vec);
            }
            "sudp" => {
                let data: Sudp = serde_json::from_str(&row.proxy_type_value)?;
                data.insert_to_vec(&mut cfg_vec);
            }
            "xtcp" => {
                let data: Xtcp = serde_json::from_str(&row.proxy_type_value)?;
                data.insert_to_vec(&mut cfg_vec);
            }
            "tcpmux" => {
                let data: TcpMux = serde_json::from_str(&row.proxy_type_value)?;
                data.insert_to_vec(&mut cfg_vec);
            }
            _ => {}
        }
    }
    let mut final_data = vec![];
    for item in cfg_vec {
        if item != "" {
            final_data.push(item)
        }
    }
    match path {
        Some(path) => {
            let mut fd = OpenOptions::new().truncate(true)
                .create(true).write(true).open(PathBuf::from(path).join("frpc.ini"))?;
            fd.write_all(final_data.join("\n").as_bytes())?;
        }
        None => {
            let dir = app_data_dir(None);
            if !dir.exists() {
                DirBuilder::new().recursive(true).create(&dir)?;
            }
            let mut fd = OpenOptions::new().truncate(true).create(true).write(true).open(dir.join("frpc.ini"))?;
            final_data.push("\n".to_string());
            fd.write_all(final_data.join("\n").as_bytes())?;
        }
    };
    Ok(())
}

pub fn parse_cargo_toml() -> Result<Vec<String>> {
    let mut ver = vec![];
    let cargo_toml = include_str!("../Cargo.toml");
    if let Ok(ml) = cargo_toml.parse::<toml::Value>() {
        if let Some(pkg) = ml.get("package") {
            if let Some(e) = pkg.get("version") {
                match e.as_str() {
                    Some(e) => {
                        ver.push(e.to_string())
                    }
                    None => {
                        return Err(anyhow!("can not parse frper version"));
                    }
                }
            }
            if let Some(pkg) = pkg.get("metadata") {
                if let Some(pkg) = pkg.get("frp") {
                    if let Some(pkg) = pkg.as_table() {
                        for (k, v) in pkg {
                            if k == "version" {
                                if let Some(v) = v.as_str() {
                                    ver.push(v.to_string())
                                }
                            }
                            break;
                        }
                    }
                } else {
                    return Err(anyhow!("can not parse frp version"));
                }
            }
        }
    }
    Ok(ver)
}