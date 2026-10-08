## TP-Link M7200

This branch adds support for using the **M7200's indicator LEDs as a hardware status display for Rayhunter**.

The M7200 is intended to be used as a **standalone Rayhunter device**. It does not need to provide normal Wi-Fi or Internet access for Rayhunter to operate. Instead, the device runs Rayhunter locally and uses its existing indicator LEDs to provide status information.

This is particularly useful because the M7200 has no display suitable for Rayhunter's normal UI. The LEDs provide a simple way to monitor Rayhunter without relying on a connected computer, web interface, Wi-Fi client, or Internet connection.

### LED status

The M7200 exposes two useful LEDs to Linux:

- `signal2_led` — Wi-Fi indicator
- `signal3_led` — Internet indicator

They are used as follows:

| Rayhunter status | M7200 indicator |
|---|---|
| Recording | Wi-Fi LED blinks |
| Paused | Wi-Fi LED blinks |
| Warning detected | Internet LED blinks |
| Stopped | Both LEDs are off |

When Rayhunter is running in this mode, these LEDs represent **Rayhunter status rather than the M7200's normal network status**.

The M7200's normal kernel LED triggers are disabled before Rayhunter takes control of the LEDs. This prevents the modem's normal LED handling from interfering with Rayhunter's status indication.

### Implementation

The M7200 LED support is implemented in:

`daemon/src/display/tplink.rs`

The implementation:

- detects the M7200 through its `signal2_led` and `signal3_led` LED interfaces;
- takes control of those LEDs;
- disables the M7200's normal kernel LED triggers;
- maps Rayhunter's `Recording`, `Paused`, and `WarningDetected` states to the physical LEDs;
- turns the LEDs off when Rayhunter shuts down;
- leaves the existing TP-Link OLED/framebuffer handling unchanged for other TP-Link devices.

The M7200-specific hardware and installation information is documented in:

`doc/tplink-m7200.md`

# Rayhunter
![Tests](https://github.com/EFForg/rayhunter/actions/workflows/main.yml/badge.svg)

![Rayhunter Logo - An Orca taking a bite out of a cellular signal bar](https://www.eff.org/files/styles/media_browser_preview/public/banner_library/rayhunter-banner.png)

Rayhunter is a project for detecting IMSI catchers, also known as cell-site simulators or stingrays. It was first designed to run on a cheap mobile hotspot called the Orbic RC400L, but thanks to community efforts, it can [support some other devices as well](https://efforg.github.io/rayhunter/supported-devices.html).
It's also designed to be as easy to install and use as possible, regardless of your level of technical skills, and to minimize false positives. 

&rarr;  Check out the [installation guide](https://efforg.github.io/rayhunter/installation.html) to get started.

&rarr; To learn more about the aim of the project, and about IMSI catchers in general, please check out our [introductory blog post](https://www.eff.org/deeplinks/2025/03/meet-rayhunter-new-open-source-tool-eff-detect-cellular-spying). 

&rarr; For discussion, help, or to join the mattermost channel and get involved with the project and community check out the [many ways listed here](https://efforg.github.io/rayhunter/support-feedback-community.html)!

&rarr; To learn more about the project in general check out the [Rayhunter Book](https://efforg.github.io/rayhunter/).

**LEGAL DISCLAIMER:** Use this program at your own risk. We believe running this program does not currently violate any laws or regulations in the United States. However, we are not responsible for civil or criminal liability resulting from the use of this software. If you are located outside of the US please consult with an attorney in your country to help you assess the legal risks of running this program.

*Good Hunting!*
