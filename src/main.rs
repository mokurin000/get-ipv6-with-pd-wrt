use std::{env, error::Error, net::Ipv6Addr, process::Stdio, str::FromStr};

use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
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

    let pd_ip = Ipv6Addr::from_str(&prefix)?;
    let suffix_ip = Ipv6Addr::from_str(&env::var("IPV6_SUFFIX")?)?;

    let result = pd_ip.to_bits() | suffix_ip.to_bits();
    println!("{}", Ipv6Addr::from_bits(result));

    Ok(())
}
