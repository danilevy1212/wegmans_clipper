use anyhow::{Ok, Result};
use clap::Parser;
use futures::future::join_all;
use wegmans_coupons_client::command_context::WebDriverContext;
use wegmans_coupons_client::http_client::coupon_client::CouponClient;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = String::from("http://localhost:4444"), help = "Webdriver server url")]
    webdriver_url: String,
    #[arg(long, help = "Webdriver server command", default_value_t = String::from("geckodriver"))]
    webdriver_command: String,
    #[arg(short, long, help = "Wegmans user email")]
    email: String,
    #[arg(short, long, help = "Wegmans user password")]
    password: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let Args {
        email,
        webdriver_url,
        password,
        webdriver_command,
    } = Args::parse();

    Ok(
        WebDriverContext::new(&webdriver_url, &webdriver_command, &email, &password)
            .with_web_driver_session(|session_cookie| async move {
                let wegmans_client = CouponClient::new(&session_cookie)?;
                let coupons = wegmans_client.get_coupons().await?;

                if !coupons.items.iter().any(|c| !c.clipped) {
                    println!("No coupons to clip");
                    return Ok(());
                }

                let coupon_calls =
                    join_all(coupons.items.into_iter().filter(|c| !c.clipped).map(|c| {
                        let coupon_client = wegmans_client.clone();
                        tokio::spawn(async move {
                            println!("Clipping coupon: {c:#?}");
                            coupon_client.clip_coupon(&c.id).await
                        })
                    }))
                    .await;

                println!("\n\n");

                for call in coupon_calls.into_iter() {
                    if call.is_err() {
                        println!("{:#?}", call.err().unwrap());
                        continue;
                    }

                    let coupon_dto = call.unwrap();

                    if coupon_dto.is_err() {
                        println!("{:#?}", coupon_dto.err().unwrap());
                        continue;
                    }

                    println!("New coupon clipped: {:#?}", coupon_dto.unwrap());
                }

                Ok(())
            })
            .await?,
    )
}
