use std::{env::args, error::Error, net::Ipv6Addr, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let pd_prefix;

    #[cfg(not(target_os = "linux"))]
    {
        pd_prefix = "ffff:ffff:ffff:ffff::";
    }
    #[cfg(target_os = "linux")]
    {
        const MAGIC_COMMAND: &str =
            r#"ubus call network.interface.wan_6 status | jq -r '.["ipv6-prefix"][0].address'"#;
        let output = std::process::Command::new("/bin/sh")
            .arg("-c")
            .arg(MAGIC_COMMAND)
            .spawn()?
            .wait_with_output()?
            .stdout;
        pd_prefix = String::from_utf8_lossy(&output);
    }

    let pd_ip = Ipv6Addr::from_str(pd_prefix.trim())?;

    let ipv6_suffix = args()
        .nth(1)
        .expect(&format!("usage: {} <ipv6_suffix>", args().nth(0).unwrap()));
    let suffix_ip = Ipv6Addr::from_str(&ipv6_suffix)?;

    let result = pd_ip.to_bits() | suffix_ip.to_bits();
    println!("{}", Ipv6Addr::from_bits(result));

    Ok(())
}
