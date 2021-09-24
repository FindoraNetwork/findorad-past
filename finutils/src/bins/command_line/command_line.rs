use clap::{App, load_yaml, crate_authors};
use std::{fmt, fs};
use finutils::common::*;

fn main() {
    if let Err(e) = run() {
        tip_fail(e);
    } else {
        tip_success();
    }
}

enum ErrorStr {
    MustSetSecKey,
    MustSetPubKey,
    MustSetAmount,
    MustSetMemo,
    MustSetAddr,
    MustSetAssetType,
}

impl ErrorStr {
    fn to_string(&self) -> String{
        match self {
            ErrorStr::MustSetSecKey => "must set seckey".to_string(),
            ErrorStr::MustSetPubKey => "must set pubkey".to_string(),
            ErrorStr::MustSetAmount => "must set amount".to_string(),
            ErrorStr::MustSetMemo => "must set memo".to_string(),
            ErrorStr::MustSetAddr => "must set addr".to_string(),
            ErrorStr::MustSetAssetType => "must set asset type, just input one number(0~255)".to_string(),
        }
    }
}

fn run() -> Result<(),String>{
    use ErrorStr::*;

    let yaml = load_yaml!("command_line.yml");

    let matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .get_matches();

    let mut hit_flag = false;

    if matches.is_present("genkey") {
        hit_flag = true;
    }

    if let Some(am) = matches.subcommand_matches("asset") {
        hit_flag = true;

        let mut is_hit = false;

        // if am.is_present("create") {
        //     is_hit = true;
        //
        //     let path = am.value_of("seckey").ok_or(MustSetSecKey.to_string())?;
        //     let from = fs::read_to_string(path).map_err(|e|e.to_string())?;
        //     let amount = am.value_of("amount").map_or_else(
        //         ||None,
        //         |d|{
        //             let r = d.parse::<u64>().map_err(|e|e.to_string()).unwrap();
        //             Some(r)
        //         }).unwrap();
        //
        //     let asset_type = am.value_of("asset_type").map_or_else(
        //         ||None,
        //         |d|{
        //             let r = d.parse::<u8>().map_err(|e|e.to_string()).unwrap();
        //             Some(r)
        //         }).unwrap();
        //
        //     create_asset(
        //         Some(&*from),
        //         amount,
        //         asset_type
        //     );
        //     return Ok(());
        // }

        if am.is_present("issue") {
            is_hit = true;

            let path = am.value_of("seckey").ok_or(MustSetSecKey.to_string())?;
            let from = fs::read_to_string(path).map_err(|e|e.to_string())?;
            let amount = am.value_of("amount").map_or_else(
                ||None,
                |d|{
                    let r = d.parse::<u64>().map_err(|e|e.to_string()).unwrap();
                    Some(r)
                }).unwrap();

            let asset_type = am.value_of("asset_type").map_or_else(
                ||None,
                |d|{
                    let r = d.parse::<u8>().map_err(|e|e.to_string()).unwrap();
                    Some(r)
                }).unwrap();

            issue_asset(
                &*from,
                amount,
                asset_type,
            );

            return Ok(());
        }

        // if am.is_present("show") {
        //     is_hit = true;
        //
        //     let addr = am.value_of("addr").ok_or(MustSetAddr.to_string())?;
        //
        //     return Ok(());
        // }

        if !is_hit {
            let help = "fn asset [--create | --issue | --show]";
            println!("{}",help);
            return Ok(());
        }
    }

    if let Some(am) = matches.subcommand_matches("transfer") {
        hit_flag = true;

        let path = am.value_of("seckey").ok_or(MustSetSecKey.to_string())?;
        let from = fs::read_to_string(path).map_err(|e|e.to_string())?;
        let asset_type = am.value_of("asset_type").map_or_else(
            ||None,
            |d|{
                let r = d.parse::<u8>().map_err(|e|e.to_string()).unwrap();
                Some(r)
            }).unwrap();
        let to = am.value_of("to-pubkey").ok_or(MustSetPubKey.to_string())?;
        let amount = am.value_of("amount").map_or_else(
            ||None,
            |d|{
                let r = d.parse::<u64>().map_err(|e|e.to_string()).unwrap();
                Some(r)
            }).unwrap();

        transfer(&*from, to, amount, asset_type);
        return Ok(());
    }

    if !hit_flag {
        println!("{}", matches.usage());
    }

    Ok(())
}

fn tip_fail(e: impl fmt::Display) {
    eprintln!("\n\x1b[31;01mFAIL !!!\x1b[00m");
    eprintln!(
        "\x1b[35;01mTips\x1b[01m:\n\tPlease send your error messages to us,\n\tif you can't understand their meanings ~^!^~\x1b[00m"
    );
    eprintln!("\n{}", e);
}

fn tip_success() {
    println!(
        "\x1b[35;01mNote\x1b[01m:\n\tYour operations has been executed without local error,\n\tbut the final result may need an asynchronous query.\x1b[00m"
    );
}

#[test]
fn test(){

}