/********************************************************************************
 *   DynHost updater for OVH                                                    *
 *                                                                              *
 *   Copyright (C) 2020 Jan Christian Grünhage                                  *
 *                                                                              *
 *   This program is free software: you can redistribute it and/or modify       *
 *   it under the terms of the GNU Affero General Public License as             *
 *   published by the Free Software Foundation, either version 3 of the         *
 *   License, or (at your option) any later version.                            *
 *                                                                              *
 *   This program is distributed in the hope that it will be useful,            *
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of             *
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the               *
 *   GNU Affero General Public License for more details.                        *
 *                                                                              *
 *   You should have received a copy of the GNU Affero General Public License   *
 *   along with this program.  If not, see <https://www.gnu.org/licenses/>.     *
 ********************************************************************************/
use custom_error::custom_error;
use reqwest::Client;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::Duration;
use std::str::FromStr;
use log::{trace, info, error};

mod config;
use config::{setup_clap, setup_fern, read_config, Config};

custom_error! {pub AddrFetchError
    Http{source: reqwest::Error}           = "Unable to get IP from ident.me: {source}",
    Addr{source: std::net::AddrParseError} = "Unable to parse IP fetched from ident.me: {source}",
}

custom_error! {pub AddrSetError
    Http{source: reqwest::Error} = "Unable to talk to OVH: {source}",
    Ovh                          = "OVH didn't like what we had to say",
}

custom_error! {pub AppError
    Addr{source: AddrFetchError} = "Unable to fetch IP",
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let clap = setup_clap();
    setup_fern(clap.occurrences_of("v"));
    let config = match read_config(clap.value_of("config").unwrap()) {
        Ok(config) => config,
        Err(error) => {
            error!("{}", error);
            return Err(());
        }
    };

    info!("Initialization done, now starting dynamic updating");

    run(config).await;
    Ok(())
}

async fn run(config: Config) -> Result<(), AppError> {
    let mut client = Client::new();

    let mut old_ipv4 = Ipv4Addr::new(127, 0, 0, 1);
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        let new_ipv4 = ipv4(&mut client).await?;
        trace!("fetched new IP: {}", new_ipv4);
        if old_ipv4 != new_ipv4 {
            trace!("IP has changed, try to update it now");
            match update_ip(&mut client, &config.hostname, &new_ipv4.to_string(), &config.username, &config.password).await {
                Ok(_) => {
                    old_ipv4 = new_ipv4;
                    info!("Successfully set new IP ({})", new_ipv4);
                }
                Err(error) => error!("Couldn't update IP: {}", error),
            }
        } else {
            trace!("IP has not changed");
        }
    }
}

async fn update_ip(
    client: &mut Client,
    hostname: &str,
    ipv4: &str,
    username: &str,
    password: &str,
) -> Result<reqwest::Response, AddrSetError> {
    let response = client
        .get("https://www.ovh.com/nic/update")
        .query(&[("system", "dyndns"), ("hostname", hostname), ("myip", ipv4)])
        .basic_auth(username, Some(password))
        .send()
        .await?;
    if !response.status().is_success() {
        error!("Couldn't update IP: {}, {}", response.status(), response.text().await?);
        Err(AddrSetError::Ovh)
    } else {
        Ok(response)
    }
}

async fn ipv4(client: &mut Client) -> Result<Ipv4Addr, AddrFetchError> {
    let ip_string = client
        .get("https://v4.ident.me/")
        .send()
        .await?
        .text()
        .await?;
    Ok(Ipv4Addr::from_str(&ip_string)?)
}

async fn ipv6(client: Client) -> Result<Ipv6Addr, AddrFetchError> {
    let ip_string = client
        .get("https://v6.ident.me/")
        .send()
        .await?
        .text()
        .await?;
    Ok(Ipv6Addr::from_str(&ip_string)?)
}
