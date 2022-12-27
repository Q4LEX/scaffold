use tracing::trace;

use crate::{input::InputHelper, settings::Settings};

pub struct Client {
    pub settings: Settings,
    pub input_helper: InputHelper,
}

impl Client {
    pub fn new() -> Self {
        let settings = Settings::read_from_file();
        let input_helper = InputHelper::new();

        Self {
            settings,
            input_helper,
        }
    }

    pub fn tick(&mut self) {
        // IMPLEMENT TICK
        // INPUT HELPER IS ALREADY BEING FED IN MAIN LOOP
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        trace!("Dropping client");
        self.settings.save_to_file();
    }
}
