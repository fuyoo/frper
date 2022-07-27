use serde::{Deserialize, Serialize};

fn generate_string(key: &str, value: &Option<String>) -> String {
    if value.is_some() {
        format!("{}={}", key, value.clone().unwrap_or("".to_string()))
    } else {
        format!("")
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Base {
    pub use_encryption: Option<String>,
    pub use_compression: Option<String>,
    pub proxy_protocol_version: Option<String>,
    pub bandwidth_limit: Option<String>,
}

impl Base {
    pub fn insert_to_vec(&self, vec: &mut Vec<String>) {
        vec.push(generate_string("use_encryption", &self.use_encryption));
        vec.push(generate_string("use_compression", &self.use_compression));
        vec.push(generate_string("proxy_protocol_version", &self.proxy_protocol_version));
        vec.push(generate_string("bandwidth_limit", &self.bandwidth_limit));
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Local {
    pub local_ip: Option<String>,
    pub local_port: Option<String>,
    pub plugin: Option<String>,
    pub plugin_params: Option<String>,
}

impl Local {
    pub fn insert_to_vec(&self, vec: &mut Vec<String>) {
        vec.push(generate_string("local_ip", &self.local_ip));
        vec.push(generate_string("local_port", &self.local_port));
        vec.push(generate_string("plugin", &self.plugin));
        vec.push(generate_string("plugin_params", &self.plugin_params));
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Slb {
    pub group: Option<String>,
    pub group_key: Option<String>,
    pub health_check_type: Option<String>,
    pub health_check_timeout_s: Option<String>,
    pub health_check_max_failed: Option<String>,
    pub health_check_interval_s: Option<String>,
    pub health_check_url: Option<String>,
}

impl Slb {
    pub fn insert_to_vec(&self, vec: &mut Vec<String>) {
        vec.push(generate_string("group", &self.group));
        vec.push(generate_string("group_key", &self.group_key));
        vec.push(generate_string("health_check_type", &self.health_check_type));
        vec.push(generate_string("health_check_timeout_s", &self.health_check_timeout_s));
        vec.push(generate_string("health_check_max_failed", &self.health_check_max_failed));
        vec.push(generate_string("health_check_interval_s", &self.health_check_interval_s));
        vec.push(generate_string("health_check_url", &self.health_check_url));
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tcp {
    pub remote_port: Option<String>,
}

impl Tcp {
    pub fn insert_to_vec(&self, vec: &mut Vec<String>) {
        vec.push(generate_string("remote_port", &self.remote_port));
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Udp {
    pub remote_port: Option<String>,
}

impl Udp {
    pub fn insert_to_vec(&self, vec: &mut Vec<String>) {
        vec.push(generate_string("remote_port", &self.remote_port));
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Http {
    pub custom_domains: Option<String>,
    pub subdomain: Option<String>,
    pub locations: Option<String>,
    pub route_by_http_user: Option<String>,
    pub http_user: Option<String>,
    pub http_pwd: Option<String>,
    pub host_header_rewrite: Option<String>,
    pub headers: Option<String>,
}

impl Http {
    pub fn insert_to_vec(&self, vec: &mut Vec<String>) {
        vec.push(generate_string("custom_domains", &self.custom_domains));
        vec.push(generate_string("subdomain", &self.subdomain));
        vec.push(generate_string("locations", &self.locations));
        vec.push(generate_string("route_by_http_user", &self.route_by_http_user));
        vec.push(generate_string("http_user", &self.http_user));
        vec.push(generate_string("http_pwd", &self.http_pwd));
        vec.push(generate_string("host_header_rewrite", &self.host_header_rewrite));
        vec.push(generate_string("headers", &self.headers));
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Https {
    pub custom_domains: Option<String>,
    pub subdomain: Option<String>,
}

impl Https {
    pub fn insert_to_vec(&self, vec: &mut Vec<String>) {
        vec.push(generate_string("custom_domains", &self.custom_domains));
        vec.push(generate_string("subdomain", &self.subdomain));
    }
}

pub trait InsertToVec {}

#[derive(Deserialize, Serialize, Debug)]
pub struct Stcp {
    pub role: Option<String>,
    pub sk: Option<String>,
}

impl Stcp {
    pub fn insert_to_vec(&self, vec: &mut Vec<String>) {
        vec.push(generate_string("role", &self.role));
        vec.push(generate_string("sk", &self.sk));
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sudp {
    pub role: Option<String>,
    pub sk: Option<String>,
}

impl Sudp {
    pub fn insert_to_vec(&self, vec: &mut Vec<String>) {
        vec.push(generate_string("role", &self.role));
        vec.push(generate_string("sk", &self.sk));
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Xtcp {
    pub role: Option<String>,
    pub sk: Option<String>,
}

impl Xtcp {
    pub fn insert_to_vec(&self, vec: &mut Vec<String>) {
        vec.push(generate_string("role", &self.role));
        vec.push(generate_string("sk", &self.sk));
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TcpMux {
    pub multiplexer: Option<String>,
    pub custom_domains: Option<String>,
    pub subdomain: Option<String>,
    pub route_by_http_user: Option<String>,
}

impl TcpMux {
    pub fn insert_to_vec(&self, vec: &mut Vec<String>) {
        vec.push(generate_string("multiplexer", &self.multiplexer));
        vec.push(generate_string("custom_domains", &self.custom_domains));
        vec.push(generate_string("subdomain", &self.subdomain));
        vec.push(generate_string("route_by_http_user", &self.route_by_http_user));
    }
}