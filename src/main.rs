use std::{error::Error, net::Ipv6Addr, process::Command, str::FromStr};

const MAGIC_COMMAND: &str =
    r#"ubus call network.interface.wan_6 status | jq -r '.["ipv6-prefix"][0].address'"#;

fn main() -> Result<(), Box<dyn Error>> {
    let pd_prefix = Command::new("/bin/sh")
        .arg("-c")
        .arg(MAGIC_COMMAND)
        .spawn()?
        .wait_with_output()?
        .stdout;
    let pd_prefix = String::from_utf8_lossy(&pd_prefix);
    let pd_ip = Ipv6Addr::from_str(&pd_prefix)?;
    println!("{pd_ip}");

    Ok(())
}
