use std::{error::Error, net::Ipv6Addr, str::FromStr};

use argh::FromArgs;
use get_ipv6_with_pd_wrt::get_ipv6_pd_from_ubus;

#[derive(FromArgs)]
#[argh(description = "main CLI interface")]
struct Cli {
    #[argh(
        option,
        description = "specify a prefix, rather than getting from IPv6-PD"
    )]
    prefix: Option<String>,
    #[argh(option, description = "suffix to merge")]
    suffix: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Cli { prefix, suffix } = argh::from_env();
    let prefix_ip = if let Some(prefix) = prefix {
        Ipv6Addr::from_str(&prefix)?
    } else {
        get_ipv6_pd_from_ubus()?
    };

    let suffix_ip = Ipv6Addr::from_str(&suffix)?;

    let result = prefix_ip.to_bits() | suffix_ip.to_bits();
    println!("{}", Ipv6Addr::from_bits(result));

    Ok(())
}
