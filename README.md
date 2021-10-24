# Fan Control

This is a fan control for a Raspberry PI based on the SYSFS interface.
It should be compatible with any SYSFS compatible system which exposes the GPIO pins and thermal zones.

## Hardware Setup

![Raspberry Pi Board Pin Overview](https://www.raspberrypi.com/documentation/computers/images/GPIO-Pinout-Diagram-2.png)

![Raspberry Pi 40 Pin GPIO Pinout](https://www.raspberrypi.com/documentation/computers/images/GPIO.png)

I'm using these fan heatsinks for my Pis: [Amazon](https://www.amazon.de/dp/B07VD6NC8P/ref=cm_sw_em_r_mt_dp_THBAEA29Y04XN7V86SXB) / [Reichelt](https://www.reichelt.de/gehaeuse-fuer-raspberry-pi-4-alu-luefter-schwarz-rpi-case-alu07f-p261678.html?)

You connect the red wires from the fans to one of the two red pins from the Pi.
The black wire from the fan gets cut in the middle and the part connected to the fan gets soldered to the *emitter* pin of a transistor and the part leading to the Pi to the *collector* pin of the transistor.
Finally connect the *base* pin of the transistor to any GPIO pin (I used 23 and grabbed ground from the one next to it) with a resistor in between (I used a 1k Ohm one).

## Software Setup

### Get Binary

#### Download Release for your system

I currently publish 64 Bit Arm binarys, since I'm developing this for my Pi 4s.

| System              | Release                 |
| :------------------ | :---------------------- |
| Raspberry Pi 64 Bit | [fan_control_aarch64]() |

#### Self Compile

##### Install Rust (with rustup)

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

##### Download or clone Sourcecode

Download the source from either the release, or the master of this repo.

```sh
$ git clone git@github.com:Snapstromegon/fan_control.git
$ cd fan_control
```

##### Compile

Inside of the project folder run:

```sh
$ cargo build --release
```

(Resulting binary will be $pwd/target/aarch64-unknown-linux-gnu/release/fan_control)

### Copy Binary and Systemd service

Copy the fan_control bin to _/opt/fan_control_ and the _fan_control.service_ (example in this repo) to _/etc/systemd/system/fan_control.service_.

### Configure

You can either configure the program via anfironment variables or command line arguments (for both, check `fan_control --help`).

You can configure:

- **GPIO Pin:** The pin which is used to turn the fan on/off
- **Thermometer Path:** SysFS path of the thermometer (should result in °C\*1000 - so 40000 = 40°C)
- **Fan On Temperature:** When should the fan kick in
- **Fan Off Temperature:** When should the fan turn off again
- **Log Level:** Use the *RUST_LOG* environment variable to change the log level. The default is "info".

### Enable Systemd service

```sh
$ systemctl enable --now fan_control
```
