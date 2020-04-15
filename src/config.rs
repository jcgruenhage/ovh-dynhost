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
use clap::{clap_app, crate_authors, crate_description, crate_name, crate_version};
use log::info;
use serde::{Deserialize, Serialize};
use custom_error::custom_error;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Config {
    pub(crate) hostname: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

pub(crate) fn setup_clap() -> clap::ArgMatches<'static> {
    clap_app!(myapp =>
        (name: crate_name!())
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg config: +required "Set config file")
        (@arg v: -v --verbose ... "Be verbose (you can add this up to 4 times for more logs).
By default, only errors are logged, so no output is a good thing.")
    )
    .get_matches()
}

pub(crate) fn setup_fern(level: u64) {
    let level = match level {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    match fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()
    {
        Err(_) => {
            eprintln!("error setting up logging!");
        }
        _ => info!("logging set up properly"),
    }
}

pub(crate) fn read_config(path: &str) -> Result<Config, Error> {
    let config_file_content = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&config_file_content)?)
}

custom_error! {pub(crate) Error
    Io{source: std::io::Error} = "Something went wrong reading the file",
    Toml{source: toml::de::Error} = "Something went wrong parsing the file"
}
