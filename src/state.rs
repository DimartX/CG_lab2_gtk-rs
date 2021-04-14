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
    pub moveFigureOx: i32,
    pub moveFigureOy: i32,
    pub moveFigureOz: i32,
    pub rotateFigureOx: i32,
    pub rotateFigureOy: i32,
    pub rotateFigureOz: i32,
    pub zoom: i32,
    pub view: View,
}
// And i really sorry about camel case

impl State {
    pub fn new(buttons: &HashMap<String, gtk::SpinButton>) -> Self {
        State {
            moveFigureOx: buttons.get("moveFigureOx").unwrap().get_value_as_int(),
            moveFigureOy: buttons.get("moveFigureOy").unwrap().get_value_as_int(),
            moveFigureOz: buttons.get("moveFigureOz").unwrap().get_value_as_int(),
            rotateFigureOx: buttons.get("rotateFigureOx").unwrap().get_value_as_int(),
            rotateFigureOy: buttons.get("rotateFigureOy").unwrap().get_value_as_int(),
            rotateFigureOz: buttons.get("rotateFigureOz").unwrap().get_value_as_int(),
            zoom:          buttons.get("zoom").unwrap().get_value_as_int(),
        }
    }

    pub fn default(&mut self) {
        self.moveFigureOx = 0;
        self.moveFigureOy = 0;
        self.moveFigureOz = 0;
        self.rotateFigureOx = 0;
        self.rotateFigureOy = 0;
        self.rotateFigureOz = 0;
        self.zoom = 100;
    }
}
