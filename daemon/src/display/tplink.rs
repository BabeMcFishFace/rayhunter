use log::{error, info};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use std::fs;
use std::time::Duration;

use crate::config::{self, UiLevel};
use crate::display::{DisplayState, tplink_framebuffer, tplink_onebit};

const WIFI_LED: &str = "/sys/class/leds/signal2_led";
const INTERNET_LED: &str = "/sys/class/leds/signal3_led";

fn is_m7200_led_device() -> bool {
    fs::exists(WIFI_LED).unwrap_or(false) && fs::exists(INTERNET_LED).unwrap_or(false)
}

async fn set_led(path: &str, brightness: u8) {
    let trigger = format!("{path}/trigger");
    let brightness_path = format!("{path}/brightness");

    // Do not use the kernel "timer" trigger.
    // The M7200's timer trigger can leave the LED at brightness 0.
    if let Err(e) = tokio::fs::write(&trigger, "none").await {
        error!("failed to disable LED trigger for {path}: {e}");
        return;
    }

    if let Err(e) = tokio::fs::write(&brightness_path, brightness.to_string()).await {
        error!("failed to set LED brightness for {path}: {e}");
    }
}

async fn all_leds_off() {
    set_led(WIFI_LED, 0).await;
    set_led(INTERNET_LED, 0).await;
}

fn update_m7200_led_ui(
    task_tracker: &TaskTracker,
    shutdown_token: CancellationToken,
    mut ui_update_rx: mpsc::Receiver<DisplayState>,
) {
    task_tracker.spawn(async move {
        let mut state = DisplayState::Recording;
        let mut led_on = false;

        info!("M7200 LED UI started");

        loop {
            if shutdown_token.is_cancelled() {
                info!("received M7200 LED UI shutdown");
                all_leds_off().await;
                break;
            }

            // Check for a new UI state without blocking the blink loop.
            match ui_update_rx.try_recv() {
                Ok(new_state) => {
                    state = new_state;
                    led_on = false;

                    // Immediately turn both LEDs off before starting
                    // the new state.
                    all_leds_off().await;

                    match state {
                        DisplayState::Recording => {
                            info!("M7200 LEDs: recording");
                        }

                        DisplayState::Paused => {
                            info!("M7200 LEDs: paused");
                        }

                        DisplayState::WarningDetected { .. } => {
                            info!("M7200 LEDs: WARNING DETECTED");
                        }
                    }
                }

                Err(mpsc::error::TryRecvError::Empty) => {}

                Err(mpsc::error::TryRecvError::Disconnected) => {
                    info!("M7200 LED UI channel disconnected");
                    all_leds_off().await;
                    break;
                }
            }

            // Toggle the appropriate LED.
            led_on = !led_on;

            match state {
                DisplayState::Recording | DisplayState::Paused => {
                    // Recording/paused:
                    // Wi-Fi LED blinks, Internet LED stays off.
                    set_led(INTERNET_LED, 0).await;

                    if led_on {
                        set_led(WIFI_LED, 255).await;
                    } else {
                        set_led(WIFI_LED, 0).await;
                    }
                }

                DisplayState::WarningDetected { .. } => {
                    // Warning:
                    // Wi-Fi LED stays off, Internet LED blinks.
                    set_led(WIFI_LED, 0).await;

                    if led_on {
                        set_led(INTERNET_LED, 255).await;
                    } else {
                        set_led(INTERNET_LED, 0).await;
                    }
                }
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });
}

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    shutdown_token: CancellationToken,
    ui_update_rx: mpsc::Receiver<DisplayState>,
) {
    // The M7200 has no screen. Its Wi-Fi and Internet LEDs are therefore
    // used as the Rayhunter status display.
    //
    // "Invisible" does not disable the physical M7200 LEDs.
    if is_m7200_led_device() {
        info!("detected TP-Link M7200 LED display");

        update_m7200_led_ui(task_tracker, shutdown_token, ui_update_rx);

        return;
    }

    // Preserve the original TP-Link behaviour for devices that have
    // the normal TP-Link display hardware.
    let display_level = config.ui_level;

    if display_level == UiLevel::Invisible {
        info!("Invisible mode, not spawning UI.");
    }

    if fs::exists(tplink_onebit::OLED_PATH).unwrap_or_default() {
        info!("detected one-bit display");

        tplink_onebit::update_ui(task_tracker, config, shutdown_token, ui_update_rx);
    } else {
        info!("fallback to framebuffer");

        tplink_framebuffer::update_ui(task_tracker, config, shutdown_token, ui_update_rx);
    }
}
