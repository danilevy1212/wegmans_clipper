use anyhow::{Ok, Result};
use clap::Parser;
use futures::future::join_all;
use log::{debug, error, info};
use wegmans_coupons_client::command_context::WebDriverContext;
use wegmans_coupons_client::coupons_dto::CouponDTO;
use wegmans_coupons_client::http_client::coupon_client::CouponClient;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = String::from("http://localhost:4444"), help = "Webdriver server url")]
    webdriver_url: String,
    #[arg(long, help = "Webdriver server command", default_value_t = String::from("geckodriver"))]
    webdriver_command: String,
    #[arg(long, help = "Webdriver server arguments")]
    webdriver_args: Vec<String>,
    #[arg(short, long, help = "Wegmans user email")]
    email: String,
    #[arg(short, long, help = "Wegmans user password")]
    password: String,
}

fn exit_with_coupons(coupons: Vec<CouponDTO>) -> Result<()> {
    info!("Listing clipped coupons");

    for coupon in coupons.iter() {
        info!("Coupon: {}", coupon.name);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let Args {
        email,
        webdriver_url,
        password,
        webdriver_command,
        webdriver_args,
    } = Args::parse();

    Ok(WebDriverContext::new(
        &webdriver_url,
        &webdriver_command,
        webdriver_args,
        &email,
        &password,
    )
    .with_web_driver_session(|session_cookie| async move {
        let wegmans_client = CouponClient::new(&session_cookie)?;
        let coupons = wegmans_client.get_coupons().await?;

        debug!("{} coupons found", coupons.items.len());

        if !coupons.items.iter().any(|c| !c.clipped) {
            info!("No coupons to clip");
            return exit_with_coupons(coupons.items);
        }

        let coupon_calls = join_all(
            coupons
                .items
                .clone()
                .into_iter()
                .filter(|c| !c.clipped)
                .map(|c| {
                    let coupon_client = wegmans_client.clone();
                    tokio::spawn(async move {
                        info!("Clipping coupon: {:?}", c);
                        coupon_client.clip_coupon(&c.id).await
                    })
                }),
        )
        .await;

        for call in coupon_calls.into_iter() {
            if call.is_err() {
                error!("{:?}", call.err().unwrap());
                continue;
            }

            let coupon_dto = call.unwrap();

            if coupon_dto.is_err() {
                error!("{:?}", coupon_dto.err().unwrap());
                continue;
            }

            info!("New coupon clipped: {:?}", coupon_dto.unwrap());
        }

        exit_with_coupons(coupons.items)
    })
    .await?)
}
