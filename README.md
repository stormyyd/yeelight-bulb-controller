Yeelight Bulb Controller
=======================

A tool for controlling your Yeelight bulb written in Rust.

## Background

I wrote this tool for making it easier to control my lamp.

Because of the limited size of my desk, when I use my laptop with a monitor at the same time, I can hardly get the button of my lamp. So, I can hardly control my lamp. Then I thought this lamp can be controlled by Mi Home or Yeelight app, that means I can also control it by using the API they provide.

And then I found [Yeelight WiFi Light Inter-Operation Specification](https://www.yeelight.com/download/Yeelight_Inter-Operation_Spec.pdf). With those APIs, I wrote this tool.

Now, I use this tool with [AutoHotkey](https://www.autohotkey.com/) to control my lamp by keyboard.

## Compile

First, make sure you have installed Rust.([Install Rust](https://www.rust-lang.org/tools/install))

Then:

```bash
git clone https://github.com/stormyyd/yeelight-bulb-controller.git
cd yeelight-bulb-controller
cargo build --release
```

You will get binary file in ./target/release/ .

## Usage

First, you need to enable "LAN Control" feature of your bulb by [following steps](https://www.yeelight.com/faqs/lan_control).

### Basic

```bash
# On Windows
yeelight-bulb-controller.exe [BULB_ADDRESS] [METHOD] [PARAM1] [PARAM2] ...
# On Linux or macOS
./yeelight-bulb-controller [BULB_ADDRESS] [METHOD] [PARAM1] [PARAM2] ...
```

### Examples

Toggle your bulb:

```bash
yeelight-bulb-controller.exe 192.168.1.123:55443 toggle
```

Set bright to 50 for your bulb smoothly:

```bash
yeelight-bulb-controller.exe 192.168.1.123:55443 set_bright 50 smooth 500
```

Turn on your bulb smoothly:

```bash
yeelight-bulb-controller.exe 192.168.1.123:55443 set_power on smooth 500
```

Turn off your bulb smoothly:

```bash
yeelight-bulb-controller.exe 192.168.1.123:55443 set_power off smooth 500
```

For more usage, please read [Yeelight WiFi Light Inter-Operation Specification](https://www.yeelight.com/download/Yeelight_Inter-Operation_Spec.pdf).

### With AutoHotKey

See [yeelight-bulb-controller.ahk](/ahk/yeelight-bulb-controller.ahk) and [Quick Reference | AutoHotkey](https://www.autohotkey.com/docs/AutoHotkey.htm).

## License

[ISC License](LICENSE) © Yingdong Yang
