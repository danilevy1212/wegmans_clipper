# Wegmans Digital Coupons Auto-Clipper

## Description

`wegmans_clipper` is a collection of utilities that automate clipping [https://shop.wegmans.com/shop/coupons](Digital Coupons) from the Wegmans website. 

The main utility is rust binary that connects to the Wegmans HTTP API, logins on the user behalf and clips any digital coupons that are unclipped. This binary will execute and connect to any [https://www.selenium.dev/documentation/webdriver](webdriver) capable program for the login portion. [https://github.com/mozilla/geckodriver](GeckoDriver) is used by default.

For ease of use, docker images are provided at the [./docker/](docker) folder:

- [./docker/Dockerfile.one-shot](one-shot): Provides all dependencies and calls the binary.
- [./docker/Dockerfile.cronjob](cronjob): Wraps the binary in a cronjob, which is called on a set schedule.

See [./compose.yaml](compose.yaml) for how to build these images.

### WARNING

The docker images use some unsafe trickery to hold on to the provided credentials. Be careful to not publish the built images to any public registry.

## Pre-requistes

- A WebDriver compatible web client.

## How to use

### Binary

```bash
Usage: wegmans_clipper [OPTIONS] --email <EMAIL> --password <PASSWORD>

Options:
      --webdriver-url <WEBDRIVER_URL>          Webdriver server url [default: http://localhost:4444]
      --webdriver-command <WEBDRIVER_COMMAND>  Webdriver server command [default: geckodriver]
      --webdriver-args <WEBDRIVER_ARGS>        Webdriver server arguments
  -e, --email <EMAIL>                          Wegmans user email
  -p, --password <PASSWORD>                    Wegmans user password
  -h, --help                                   Print help
```

### Docker Compose

Cronjob

```bash
docker compose up coupons_cronjob
```

One-shot

```bash
docker compose up coupons_oneshot
```

## Why are you using a headless browser and HTTP calls?

Originally, I used the WebDriver server to get through the authentication flow. This allowed me to iterate easily to reverse engineer the rest to HTTP calls. However, when it came to figure out how to do login worked using HTTP only, I kept getting banned by cloudflare's anti web-crawling measures. Finally, after much tinkering around, I gave up. Though the final result is a frankenstein monster of HTTP clients and headless browsers that I am not totally happy about, it does it's job.

The docker recipies provided help contain the complexity introduced by the dependency with WebDriver. Building and using the Dockerfiles is the intended use.
