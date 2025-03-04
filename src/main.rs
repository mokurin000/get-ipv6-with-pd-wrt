use std::{env, error::Error, net::Ipv6Addr, str::FromStr};

const MAGIC_COMMAND: &str =
    r#"ubus call network.interface.wan_6 status | jq -r '.["ipv6-prefix"][0].address'"#;

fn main() -> Result<(), Box<dyn Error>> {
    let output = std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg(MAGIC_COMMAND)
        .spawn()?
        .wait_with_output()?
        .stdout;
    let pd_prefix = String::from_utf8_lossy(&output).to_string();

    let pd_ip = Ipv6Addr::from_str(pd_prefix.trim())?;
    let suffix_ip = Ipv6Addr::from_str(&env::var("IPV6_SUFFIX")?)?;

    let result = pd_ip.to_bits() | suffix_ip.to_bits();
    println!("{}", Ipv6Addr::from_bits(result));

    Ok(())
}
