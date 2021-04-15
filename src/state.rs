use std::collections::HashMap;
use gtk::prelude::*;

pub enum View {
    Isometric,
    Side,
    Above,
    Front,
}

// Shared state for communication between buttons and drawingarea
pub struct State {
    pub stretchOx: f64,
    pub stretchOy: f64,
    pub stretchOz: f64,
    pub moveOx:    f64,
    pub moveOy:    f64,
    pub moveOz:    f64,
    pub rotateOx:  f64,
    pub rotateOy:  f64,
    pub rotateOz:  f64,
    pub zoom:      f64,
    pub view:      View,
}
// And i really sorry about camel case

impl State {
    pub fn new(buttons: &HashMap<String, gtk::SpinButton>) -> Self {
        State {
            stretchOx: buttons.get("stretchOx").unwrap().get_value(),
            stretchOy: buttons.get("stretchOy").unwrap().get_value(),
            stretchOz: buttons.get("stretchOz").unwrap().get_value(),
            moveOx:    buttons.get("moveOx")   .unwrap().get_value(),
            moveOy:    buttons.get("moveOy")   .unwrap().get_value(),
            moveOz:    buttons.get("moveOz")   .unwrap().get_value(),
            rotateOx:  buttons.get("rotateOx") .unwrap().get_value(),
            rotateOy:  buttons.get("rotateOy") .unwrap().get_value(),
            rotateOz:  buttons.get("rotateOz") .unwrap().get_value(),
            zoom:      buttons.get("zoom")     .unwrap().get_value(),
            view:      View::Isometric,
        }
    }

    pub fn default(&mut self) {
        self.stretchOx = 100.0;
        self.stretchOy = 100.0;
        self.stretchOz = 100.0;
        self.moveOx    = 0.0;
        self.moveOy    = 0.0;
        self.moveOz    = 0.0;
        self.rotateOx  = 0.0;
        self.rotateOy  = 0.0;
        self.rotateOz  = 0.0;
        self.zoom    = 100.0;
        self.view = View::Isometric;
    }
}
