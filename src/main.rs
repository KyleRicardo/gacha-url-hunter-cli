use std::error::Error;

use arboard::Clipboard;
use inquire::Select;
use spinners::{Spinner, Spinners};

use gacha_url_hunter::common::{GameRegion, MihoyoGame};
use gacha_url_hunter::hunter::hunt_gacha_url;

mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let game = Select::new("选择游戏 Select game:", MihoyoGame::vec()).prompt()?;
    let region = Select::new("选择区服 Select region:", GameRegion::vec()).prompt()?;
    let mut sp = Spinner::new(Spinners::Dots, "正在获取抽卡链接...".into());
    let url = hunt_gacha_url(game, region);
    match url {
        Ok(url) => {
            sp.stop_and_persist("\x1b[32m✔\x1b[0m", "获取完成".into());
            println!("您的抽卡分析链接为：");
            println!("{}", url);
            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(url)?;
            println!("抽卡链接已复制到剪贴板");
        }
        Err(e) => {
            sp.stop_and_persist("\x1b[31m✖\x1b[0m", "获取失败".into());
            eprintln!("{}", e)
        }
    }

    util::pause();

    Ok(())
}
