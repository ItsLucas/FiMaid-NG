use std::collections::HashMap;

use chrono::Local;
use reqwest::{self, Error, Response};
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

const API_ROOT: &str = "http://192.168.0.1";
const USB_PORT: &str = "1-1";
const ADMIN_PASSWORD: &str = "YWRtaW4%3D";
lazy_static! {
    static ref LOCKED: Mutex<usize> = Mutex::const_new(0);
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let url = reqwest::Url::parse("http://raburaibu.me0w.men").unwrap();
    let bot = Bot::from_env().set_api_url(url);

    MyBotCommand::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "我有这些功能哦喵~")]
enum MyBotCommand {
    #[command(description = "告诉主人怎么使用本尊（坏耶不可以）")]
    Help,
    #[command(description = "开启Google Fi")]
    On(String),
    #[command(description = "关闭Google Fi")]
    Off,
}

async fn set_cmd_process(func: &str, field: &str, value: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client.post(API_ROOT.to_string() + "/goform/goform_set_cmd_process")
        .body(format!("goformId={func}&{field}={value}"))
        .timeout(Duration::from_secs(2))
        .send()
        .await
}

async fn get_cmd_process(fields: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let url = format!("/goform/goform_get_cmd_process?multi_data=1&sms_received_flag_flag=0&sts_received_flag_flag=0&cmd={fields}");
    client.get(API_ROOT.to_string() + &url).timeout(Duration::from_secs(2)).send().await
}

async fn poweroff(bot: Bot, msg: Message) {
    let _ = LOCKED.lock().await;
    bot.send_message(msg.chat.id, format!("收到喵~ 正在关闭 Google Fi")).await.unwrap();
    let mut child  = Command::new("uhubctl")
        .arg("-l").arg(USB_PORT)
        .arg("-a").arg("0")
        .spawn().expect("Failed to spawn uhubctl!");
    let status = child.wait().await.expect("Failed to wait for uhubctl!");
    if !status.success() {
        warn!("Failed to poweroff usb port!");
        bot.send_message(msg.chat.id, "USB端口不知道怎么关不掉了呜呜 （；へ：）").await.unwrap();
    } else {
        for _ in 1..6 {
            info!("Quering for dongle status...");
            let status_resp = get_cmd_process("ppp_status%2Cnetwork_type")
                .await;
            if let Err(_) = status_resp {
                info!("Dongle turned off.");
                bot.send_message(msg.chat.id, "喵~主人，Google Fi 已经为主人关掉了哦").await.unwrap();
                return;
            }
        }
        warn!("Failed to poweroff usb port! Panel still online.");
        bot.send_message(msg.chat.id, "USB端口不知道怎么关不掉了呜呜，好像面板也还可以访问，主人怎么办嘛怎么办嘛 （；へ：）").await.unwrap();
    }
}

async fn answer(bot: Bot, msg: Message, cmd: MyBotCommand) -> ResponseResult<()> {
    match cmd {
        MyBotCommand::Help => bot.send_message(msg.chat.id, MyBotCommand::descriptions().to_string()).await?,
        MyBotCommand::On(duration) => {
            let mut hours = duration.parse::<u64>().unwrap_or(3);
            if hours > 12 {
                hours = 3;
            }
            
            // Query current status
            info!("Quering for dongle status...");
            let login_result = set_cmd_process("LOGIN", "password", ADMIN_PASSWORD)
                    .await;
            if let Ok(response) = login_result {
                let login_resp = response.json::<HashMap<String, String>>().await?;
                let login_status = login_resp.get("result").expect("result missing in the login response");
                if login_status != "0" {
                    bot.send_message(msg.chat.id, format!("网卡在线但是登录不能！好奇怪啊……能不能过来帮帮看看嘛(°ロ°) !")).await?;
                    return Ok(());
                }
                info!("Logged in.");
                let status_resp = get_cmd_process("ppp_status%2Cnetwork_type")
                    .await;
                if let Ok(response) = status_resp {
                    let json_value = response.json::<HashMap<String, String>>().await?;
                    let ppp_status = json_value.get("ppp_status").expect("ppp_status not found in the status response!");
                    info!("Current status: {ppp_status}");
                    if ppp_status == "ppp_connected" {
                        bot.send_message(msg.chat.id, format!("Google Fi已经启动了啦，笨笨主人是不是忘记了啦（揪")).await?;
                        return Ok(());
                    } else {
                        bot.send_message(msg.chat.id, format!("虽然网卡已经启动了但是好像……好像并没能连网Σ(°△°|||)︴快去排障一下")).await?;
                        return Ok(());
                    }
                }
            }
            

            // Locked to avoid duplicated request
            let _ = LOCKED.lock().await;
            bot.send_message(msg.chat.id, format!("分かりました！正在为主人开启 Google Fi 喵")).await?;
            info!("Dongle down. Trying to power on...");

            // Poweron USB port
            let mut child  = Command::new("uhubctl")
                .arg("-l").arg(USB_PORT)
                .arg("-a").arg("1")
                .spawn().expect("Failed to spawn uhubctl!");
            let status = child.wait().await?;
            if !status.success() {
                bot.send_message(msg.chat.id, "USB端口坏掉了启动不了了 ｜д•´)!!").await?;
                return Ok(());
            }

            // Since the USB dongle is booting, we have to try several times to wait it online
            info!("Trying to login to the dongle");
            for times in 1..6 {
                sleep(Duration::from_secs(5)).await;
                // Login
                let login_result = set_cmd_process("LOGIN", "password", ADMIN_PASSWORD)
                    .await;
                if let Ok(response) = login_result {
                    let login_resp = response.json::<HashMap<String, String>>().await?;
                    let login_status = login_resp.get("result").expect("result missing in the login response");
                    if login_status != "0" {
                        bot.send_message(msg.chat.id, format!("登录到网卡的网页控制端失败了！错误代码是 {login_status}，主人帮帮看看好不好了啦（摇手手")).await?;
                        return Ok(());
                    }
                    info!("Logged in.");
                    break;
                } else if times == 5 {
                    warn!("Failed to login to the dongle.");
                    bot.send_message(msg.chat.id, "无法连接到网卡惹°(°ˊДˋ°) °感觉网卡似乎通电不能坏掉惹").await?;
                    return Ok(());
                }
            }

            // Turn off wifi as we don't need it
            info!("Attempting to turn off wifi");
            let disable_wifi_resp = set_cmd_process("SET_WIFI_INFO", "wifiEnabled", "0")
                .await?
                .json::<HashMap<String, String>>()
                .await?;
            
            let disable_wifi_status = disable_wifi_resp.get("result").expect("result missing in the login response");
            if disable_wifi_status != "success" {
                // This error doesn't matter, so do not panic
                warn!("Failed to disable wifi! Return code: {disable_wifi_status}");
                bot.send_message(msg.chat.id, format!("WiFi好像关不掉了耶，错误码是 {disable_wifi_status}")).await?;
            }
            info!("Wifi is off now.");

            // Loop until ppp_status = ppp_connected & network_type = LTE
            let mut ppp_flag = false;
            let mut lte_flag = false;
            for _round in 1..6 {
                // First, have a sweet, sweet sleep for 10 seconds
                sleep(Duration::from_secs(3)).await;
                // Query current status
                let status_resp = get_cmd_process("ppp_status%2Cnetwork_type")
                    .await?
                    .json::<HashMap<String, String>>()
                    .await?;
                let ppp_status = status_resp.get("ppp_status").expect("ppp_status not found in the status response!");
                let network_type = status_resp.get("network_type").expect("network_status not found in status response!");
                info!("status: {ppp_status} {network_type}");
                if !ppp_flag {
                    if ppp_status == "ppp_connected" {
                        ppp_flag = true;
                        info!("PPP connected.");
                        bot.send_message(msg.chat.id, "PPP 连接成功！(￣▽￣)ノ").await?;
                    }
                }
                if !lte_flag {
                    if network_type == "LTE" {
                        lte_flag = true;
                        info!("LTE established.");
                        bot.send_message(msg.chat.id, "LTE 连接已建立！qwq").await?;
                    }
                }
                if ppp_flag && lte_flag {
                    break;
                }
            }

            // Set task to poweroff dongle
            let botcopy = bot.clone();
            let msgcopy = msg.clone();
            tokio::spawn(async move {
                sleep(Duration::from_secs(hours * 60 * 60)).await;
                let login_result = set_cmd_process("LOGIN", "password", ADMIN_PASSWORD)
                    .await;
                if let Err(_) = login_result {
                    return;
                }
                poweroff(botcopy, msgcopy).await;
            });

            if ppp_flag && lte_flag {
                info!("Google Fi is ready.");

                let mut delayed_hours = duration.parse::<i64>().unwrap_or(3);
                if delayed_hours > 12 {
                    delayed_hours = 3;
                }

                let now = Local::now();
                let planned_time = now + chrono::Duration::hours(delayed_hours);
                let datetime_str = planned_time.format("%Y-%m-%d %H:%M:%S").to_string();

                bot.send_message(msg.chat.id, format!("喵~ Google Fi 已经准备好啦，现在可以连接相应的节点使用 Google Fi 上网啦\n\n根据ご主人様的指示，将在 {delayed_hours} 小时后即 {datetime_str} 关闭 Google Fi 捏~")).await?
            } else {
                warn!("Google Fi failed to start.");
                bot.send_message(msg.chat.id, "呜呜果咩捏，Google Fi 启动不起来了QAQ. ご主人様可以去看看日志嘛（超小声").await?
            }
        }
        MyBotCommand::Off => {
            let botcopy = bot.clone();
            let msgcopy = msg.clone();
            poweroff(botcopy, msgcopy).await;
            return Ok(())
        },
    };

    Ok(())
}