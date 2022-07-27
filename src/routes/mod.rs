pub mod settings;

use sciter::make_args;
use crate::event::Body;
use crate::response::{Response, ResponseBody};

pub async fn dispatch(mut body: Body) -> anyhow::Result<()> {
    let res = match body.path.as_str() {
        "/settings" => settings::get_client_settings().await.into_response()?,
        "/setting/update" => settings::set_value_by_key(&mut body).await?.into_response()?,
        "/proxy" => settings::get_proxy_settings().await.into_response()?,
        "/proxy/add" => settings::proxy_settings_add(&mut body).await.into_response()?,
        "/proxy/edit" => settings::proxy_setting_edit(&mut body).await.into_response()?,
        "/proxy/delete" => settings::proxy_setting_delete(&mut body).await.into_response()?,
        "/proxy/enable" => settings::proxy_setting_toggle_enabled(&mut body).await.into_response()?,
        "/export" => settings::export_conf(body.data).await.into_response()?,
        "/version" => settings::version().await.into_response()?,
        _ => Response::<Option<String>>::new(404, None, "not found!").into_response()?
    };
    body.cb.call(None, &make_args!(res), None)?;
    Ok(())
}

