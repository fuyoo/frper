#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use std::thread::spawn;
use crate::app::get_db_conn;
use crate::response::{Response, ResponseBody};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use crate::event::Body;
use crate::service::{generate_ini_config, parse_cargo_toml};

#[derive(Deserialize, Serialize, Debug)]
pub struct Cfg {
    pub key: String,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<String>,
    pub value: String,
    pub desc: String,
    pub options: Option<String>,
    pub remark: Option<String>,
    #[serde(rename = "setType")]
    pub set_type: String,
}

pub async fn get_client_settings() -> impl ResponseBody {
    let conn = get_db_conn().await?;
    let mut stmt = conn.prepare("select * from client_setting")?;
    let rows = stmt.query_map([], |row| Ok(Cfg {
        key: row.get("key")?,
        default_value: row.get("default_value")?,
        value: row.get("value")?,
        desc: row.get("desc")?,
        options: row.get("options")?,
        remark: row.get("remark")?,
        set_type: row.get("set_type")?,
    }))?;
    let mut data = vec![];
    for row in rows {
        data.push(row?)
    }
    return Ok(data);
}

pub async fn set_value_by_key(data: &mut Body) -> Result<String> {
    let params: Cfg = serde_json::from_str(&data.data)?;
    let conn = get_db_conn().await?;
    let s = conn.execute("update client_setting set value=?1 where key=?2", [params.value, params.key])?;
    if s != 1 {
        Err(anyhow!("update failed"))
    } else {
        data.service.lock().restart()?;
        Ok("ok".to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProxyTable {
    pub id: Option<String>,
    pub name: String,
    pub base: String,
    pub local: String,
    pub slb: String,
    pub proxy_type: String,
    pub proxy_type_value: String,
    pub enabled: bool,
}

pub async fn get_proxy_settings() -> Result<Vec<ProxyTable>> {
    let conn = get_db_conn().await?;
    let mut stmt = conn.prepare("select * from proxy_setting")?;
    let rows = stmt.query_map([], |row| {
        let s: String = row.get("enabled")?;
        Ok(ProxyTable {
            id: row.get("id")?,
            name: row.get("name")?,
            base: row.get("base")?,
            local: row.get("local")?,
            slb: row.get("slb")?,
            proxy_type: row.get("proxy_type")?,
            proxy_type_value: row.get("proxy_type_value")?,
            enabled: s.as_str() == "true",
        })
    })?;
    let mut res = vec![];
    for row in rows {
        res.push(row?);
    };
    Ok(res)
}

pub async fn proxy_settings_add<'a>(body: &mut Body) -> Result<Response<'a, Option<String>>> {
    let data: ProxyTable = serde_json::from_str(&body.data)?;
    let uuid = uuid::Uuid::new_v4().to_string();
    let conn = get_db_conn().await?;
    let s = conn.execute(r#"insert into proxy_setting(id,name,base,local,slb,proxy_type,proxy_type_value,enabled)
    values(?1,?2,?3,?4,?5,?6,?7,?8)
    "#, [uuid, data.name, data.base, data.local, data.slb, data.proxy_type, data.proxy_type_value, data.enabled.to_string()])?;
    if s != 1 {
        return Ok(Response::fail(None, Some("新建失败")));
    }
    body.service.lock().restart()?;
    Ok(Response::ok(None, None))
}

pub async fn proxy_setting_edit<'a>(body: &mut Body) -> Result<Response<'a, Option<String>>> {
    let data: ProxyTable = serde_json::from_str(&body.data)?;
    let conn = get_db_conn().await?;
    let s = conn.execute(r#"update proxy_setting set name=?1, base =?2, local=?3, slb=?4, proxy_type=?5, proxy_type_value=?6 where id=?7 "#,
                         [data.name, data.base, data.local, data.slb, data.proxy_type, data.proxy_type_value, data.id.unwrap_or("-1".to_string())])?;
    if s != 1 {
        return Ok(Response::fail(None, None));
    }
    body.service.lock().restart()?;
    Ok(Response::ok(None, None))
}

pub async fn proxy_setting_toggle_enabled<'a>(body: &mut Body) -> Result<Response<'a, Option<String>>> {
    let data: ProxyTable = serde_json::from_str(&body.data)?;
    let conn = get_db_conn().await?;
    let s = conn.execute(r#"update proxy_setting set enabled = ?1 where id=?2 "#,
                         [data.enabled.to_string(), data.id.unwrap_or("-1".to_string())])?;
    if s != 1 {
        return Ok(Response::fail(None, Some("操作失败")));
    }
    body.service.lock().restart()?;
    Ok(Response::ok(None, None))
}

pub async fn proxy_setting_delete<'a>(body: &mut Body) -> Result<Response<'a, Option<String>>> {
    let conn = get_db_conn().await?;
    let s = conn.execute(r#"delete from proxy_setting where id=?1"#,
                         [body.data.to_string()])?;
    if s != 1 {
        return Ok(Response::fail(None, Some("操作失败")));
    }
    body.service.lock().restart()?;
    Ok(Response::ok(None, None))
}

pub async fn export_conf(path: String) -> Result<String> {
    generate_ini_config(Some(path.clone()))?;
    let path = PathBuf::from(path).as_path().join("frpc.ini").display().to_string();
    let in_path = path.clone();
    spawn(move || -> Result<()> {
        #[cfg(target_os = "windows")]
        Command::new("explorer.exe")
            .arg(format!("/select,{}",in_path.replace("/","\\")))
            .creation_flags(0x08000000)
            .spawn()?;
        #[cfg(target_os = "macos")]
        Command::new("open").arg("-R")
            .arg(in_path)
            .spawn()?;
        Ok(())
    });
    Ok(path)
}

pub async fn version() -> Result<Vec<String>> {
    let res = parse_cargo_toml()?;
    Ok(res)
}