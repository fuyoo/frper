use std::sync::{Arc};
use anyhow::{anyhow, Result};
use sciter::{dispatch_script_call, EventHandler, make_args, Value};
use flume::{Sender as BodySender, Receiver as BodyReceiver, RecvError};
use log::error;
use parking_lot::Mutex;

use crate::{routes};
use crate::response::{Response, ResponseBody};
use crate::service::Service;

#[derive(Clone, Debug)]
pub struct Body {
    pub path: String,
    pub data: String,
    pub cb: Value,
    pub service: Arc<Mutex<Service>>,
}

/// evt struct
pub struct Evt {
    pub body_sender: BodySender<Body>,
    pub service: Arc<Mutex<Service>>,
}

impl Evt {
    /// fetch event provide a way for front-end and back-end to exchange data.
    pub fn fetch(&mut self, path: String, data: String, cb: Value) -> Result<()> {
        self.body_sender.send(Body {
            path,
            data,
            cb,
            service: self.service.clone(),
        }).map_err(|e| anyhow!("{}",e))?;
        Ok(())
    }
    /// exit event can exit the app form front-end
    pub fn exit(&mut self) -> Result<()> {
        self.service.lock().exit()?;
        std::process::exit(0);
    }
}

impl EventHandler for Evt {
    dispatch_script_call! {
        fn fetch(String,String,Value);
        fn exit();
    }
}

/// init consume data service
pub fn do_request_services(receiver: BodyReceiver<Body>) {
    std::thread::spawn(move || {
        tokio::runtime::Runtime::new().expect("init tokio runtime failed!")
            .block_on(async {
                loop {
                    match receiver.recv_async().await {
                        Ok(action) => {
                            let act = action.clone();
                            match routes::dispatch(act).await {
                                Err(err) => {
                                    error!("{}", &err);
                                    if let Ok(data) =
                                    Response::<Option<&str>>::new(500, None, &format!("{}", err))
                                        .into_response()
                                    {
                                        let _ = action.cb.call(None, &make_args!(data), None);
                                    };
                                }
                                _ => {}
                            }
                        }
                        Err(e) => match e {
                            RecvError::Disconnected => {
                                error!("do_request_services channel disconnected");
                                break;
                            }
                        },
                    }
                }
            })
    });
}