use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, AppSettings, Arg,
};
use tempfile::NamedTempFile;
use walkdir::{DirEntry, WalkDir};
mod settings;

fn main() {
    println!("Hello, world !");

    let matches = app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::from_usage("-c --cfg [CFG_PATH] 'config file (toml) path'")
                .default_value("enkm.toml"),
        )
        .arg(Arg::from_usage("<PATH> 'log path'").default_value("."))
        .arg(Arg::from_usage("-o --output [OUTPUT_PATH] 'output path'"))
        .get_matches();

    let _cfg = matches.value_of("cfg").unwrap();
}
