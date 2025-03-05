use std::{error::Error, net::Ipv6Addr, process::Stdio, str::FromStr};

use serde_json::Value;

pub fn get_ipv6_pd_from_ubus() -> Result<Ipv6Addr, Box<dyn Error>> {
    let output = std::process::Command::new("/bin/ubus")
        .arg("call")
        .arg("network.interface.wan_6")
        .arg("status")
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?
        .stdout;

    let result_json: Value = serde_json::from_slice(&output)?;
    let prefix = result_json
        .pointer("/ipv6-prefix/0/address")
        .ok_or("failed to parse delegated prefix")?
        .as_str()
        .ok_or("delegated prefix must be string")?;

    Ok(Ipv6Addr::from_str(prefix)?)
}
