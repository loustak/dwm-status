use super::RenderConfig;
use crate::feature::Renderable;
use crate::utils::icon_by_percentage;
use crate::wrapper::battery::Battery;
use crate::wrapper::uom::get_raw_hours;
use crate::wrapper::uom::get_raw_minutes;
use crate::wrapper::uom::get_raw_percent;
use uom::si::f32::Time;

#[derive(Debug)]
pub(super) struct Data {
    cache: String,
    config: RenderConfig,
}

impl Data {
    #[allow(clippy::missing_const_for_fn)]
    pub(super) fn new(config: RenderConfig) -> Self {
        Self {
            cache: String::new(),
            config,
        }
    }

    pub(super) fn update(&mut self, batteries: &[Battery]) {
        self.cache = if batteries.is_empty() {
            self.config.no_battery.clone()
        } else {
            batteries
                .iter()
                .map(|battery| {
                    self.render_battery(battery)
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
                .join(&self.config.separator)
        }
    }

    fn render_battery(&self, battery: &Battery) -> Vec<String> {
        match *battery {
            Battery::Charging {
                percentage,
                time_to_full,
            } => {
                let capacity = get_raw_percent(percentage);

                let mut list = vec![];
                list.push(self.config.charging.to_owned());
                self.push_capacity(&mut list, capacity);
                self.push_time(&mut list, time_to_full);
                list
            },
            Battery::Discharging {
                percentage,
                time_to_empty,
            } => {
                let capacity = get_raw_percent(percentage);

                let mut list = vec![];
                list.push(self.config.discharging.to_owned());
                self.push_capacity(&mut list, capacity);
                self.push_time(&mut list, time_to_empty);
                list
            },
            Battery::Empty => {
                let mut list = vec![];
                self.push_capacity(&mut list, 0.);
                list
            },
            Battery::Full => {
                let mut list = vec![];
                self.push_capacity(&mut list, 100.);
                list
            },
        }
    }

    fn push_capacity(&self, list: &mut Vec<String>, capacity: f32) {
        let icon = icon_by_percentage(&self.config.icons, capacity);

        if let Some(icon_str) = icon {
            list.push(icon_str.to_owned());
        }

        list.push(format!("{:.0}%", capacity));
    }

    fn push_time(&self, list: &mut Vec<String>, time: Time) {
        list.push(format!(
            "({:02}:{:02})",
            get_raw_hours(time),
            get_raw_minutes(time)
        ));
    }
}

impl Renderable for Data {
    fn render(&self) -> &str {
        &self.cache
    }
}
