use std::error::Error;

use arboard::Clipboard;
use inquire::Select;
use spinners::{Spinner, Spinners};

use gacha_url_hunter::common::{ExtractOption, GameRegion, MihoyoGame};
use gacha_url_hunter::hunter::hunt_gacha_url;
use gacha_url_hunter::utils::extract_authkey;

mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let game = Select::new("选择游戏 Select game:", MihoyoGame::vec()).prompt()?;
    let region = Select::new("选择区服 Select region:", GameRegion::vec()).prompt()?;
    let extract_option = Select::new("选择提取方式 Select extract option:", ExtractOption::vec()).prompt()?;
    let mut sp = Spinner::new(Spinners::Dots, "正在获取抽卡链接...".into());
    let url = hunt_gacha_url(game, region);
    match url {
        Ok(url) => {
            sp.stop_and_persist("\x1b[32m✔\x1b[0m", "获取完成".into());
            if extract_option == ExtractOption::CompleteUrl {
                println!("您的抽卡分析链接为：");
                println!("{}", url);
                let mut clipboard = Clipboard::new().unwrap();
                clipboard.set_text(url)?;
                println!("抽卡链接已复制到剪贴板");
            } else {
                println!("您的Authkey为：");
                let authkey: String;
                if extract_option == ExtractOption::UrlEncodedAuthkeyOnly {
                    authkey = extract_authkey(url, false)?;
                } else {
                    authkey = extract_authkey(url, true)?;
                }
                println!("{}", authkey);
                let mut clipboard = Clipboard::new().unwrap();
                clipboard.set_text(authkey)?;
                println!("Authkey已复制到剪贴板");
            }
        }
        Err(e) => {
            sp.stop_and_persist("\x1b[31m✖\x1b[0m", "获取失败".into());
            eprintln!("{}", e)
        }
    }

    util::pause();

    Ok(())
}
