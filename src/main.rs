use evdev::{Device, EventSummary, KeyCode, RelativeAxisCode};
use std::{collections::HashSet, process::Command, thread::sleep, time::Duration};

const HORIZONTAL_THRESHOLD: i32 = 100;
const VERTICAL_THRESHOLD: i32 = 100;

// Calculates accumulation of left right movements
struct Accumulator {
    accum_right: i32,
    accum_left: i32,
    accum_up: i32,
    accum_down: i32,
}

impl Accumulator {
    fn add_horizontal(&mut self, value: i32) {
        if value > 0 {
            self.accum_right += value;
            self.accum_left = 0;
        } else if value < 0 {
            self.accum_left -= value;
            self.accum_right = 0;
        }
    }

    fn add_vertical(&mut self, value: i32) {
        if value < 0 {
            self.accum_up += -value;
            self.accum_down = 0;
        } else if value > 0 {
            self.accum_down += value;
            self.accum_up = 0;
        }
    }

    fn cancel_if_vertical_dominates(&mut self) {
        if (self.accum_up > self.accum_right && self.accum_up > self.accum_left)
            || (self.accum_down > self.accum_right && self.accum_down > self.accum_left)
        {
            self.accum_right = 0;
            self.accum_left = 0;
        }
    }

    fn trigger_actions(&mut self) {
        if self.accum_right > HORIZONTAL_THRESHOLD {
            view_next_dwm();
            self.accum_right = 0;
        } else if self.accum_left > HORIZONTAL_THRESHOLD {
            view_prev_dwm();
            self.accum_left = 0;
        }

        if self.accum_up > VERTICAL_THRESHOLD {
            on_next_tag_action();
            self.accum_up = 0;
        } else if self.accum_down > VERTICAL_THRESHOLD {
            on_prev_tag_action();
            self.accum_down = 0;
        }
    }
}

fn update_window(
    accumulator: &mut Accumulator,
    value: i32,
    vertical_value: i32,
    keys_pressed: &HashSet<KeyCode>,
) {
    if keys_pressed.contains(&KeyCode::BTN_TASK) {
        accumulator.add_horizontal(value);
        accumulator.add_vertical(vertical_value);
        accumulator.cancel_if_vertical_dominates();
        accumulator.trigger_actions();
    }
}

fn run_trackball_settings() -> std::io::Result<()> {
    let mut keys_pressed = HashSet::new();
    // Adjust to Trackball event
    // TODO: Auto get trackball
    let mut device = Device::open("/dev/input/event14")?;

    let mut accumulator = Accumulator {
        accum_right: 0,
        accum_left: 0,
        accum_up: 0,
        accum_down: 0,
    };

    let mut horizontal_value = 0;
    let mut vertical_value = 0;

    loop {
        sleep(Duration::from_millis(1));
        for event in device.fetch_events().unwrap() {
            match event.destructure() {
                EventSummary::Key(_ev, key, 1) => {
                    keys_pressed.insert(key);
                }
                EventSummary::Key(_ev, key, 0) => {
                    keys_pressed.remove(&key);
                }
                EventSummary::RelativeAxis(_raw_event, RelativeAxisCode::REL_X, value) => {
                    horizontal_value = value;
                    update_window(
                        &mut accumulator,
                        horizontal_value,
                        vertical_value,
                        &keys_pressed,
                    );
                    horizontal_value = 0;
                }
                EventSummary::RelativeAxis(_raw_event, RelativeAxisCode::REL_Y, value) => {
                    vertical_value = value;
                    update_window(
                        &mut accumulator,
                        horizontal_value,
                        vertical_value,
                        &keys_pressed,
                    );
                    vertical_value = 0;
                }
                _ => {}
            }
        }
    }
}

// On BTN_TASK + Right movement
fn view_next_dwm() {
    Command::new("dwm-msg")
        .args(&["run_command", "viewnext"])
        .spawn()
        .expect("Failed to execute dwm-msg");
}

// On BTN_TASK + Left movement
fn view_prev_dwm() {
    Command::new("dwm-msg")
        .args(&["run_command", "viewprev"])
        .spawn()
        .expect("Failed to execute dwm-msg");
}

// On BTN_TASK + Up movement
fn on_next_tag_action() {
    Command::new("dwm-msg")
        .args(&["run_command", "tagtonext"])
        .spawn()
        .expect("Failed to execute dwm-msg");
}

// On BTN_TASK + Down movement
fn on_prev_tag_action() {
    Command::new("dwm-msg")
        .args(&["run_command", "tagtoprev"])
        .spawn()
        .expect("Failed to execute dwm-msg");
}

fn main() -> std::io::Result<()> {
    run_trackball_settings()
}
