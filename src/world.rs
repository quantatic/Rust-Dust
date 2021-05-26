pub mod element;

pub use element::Element;

use std::convert::TryFrom;

#[derive(Debug)]
pub struct World {
    elements: Vec<Option<Element>>, // access as world[y * width + x]
    elements_back_buffer: Vec<Option<Element>>,
    width: i32,
    height: i32,
    gravity_x: i32,
    gravity_y: i32,
}

impl World {
    pub fn new(width: i32, height: i32) -> Self {
        let elements = vec![None; usize::try_from(width * height).unwrap()];
        let elements_back_buffer = vec![None; usize::try_from(width * height).unwrap()];
        Self {
            elements,
            elements_back_buffer,
            width,
            height,
            gravity_y: -1,
            gravity_x: 0,
        }
    }

    pub fn get_element(&self, x: i32, y: i32) -> &Option<Element> {
        &self.elements[usize::try_from((y * self.width) + x).unwrap()]
    }

    pub fn get_element_mut(&mut self, x: i32, y: i32) -> &mut Option<Element> {
        &mut self.elements[usize::try_from((y * self.width) + x).unwrap()]
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn tick(&mut self) {
        for x in 0..self.width() {
            for y in 0..self.height() {
                if let Some(element) = self.get_element_mut(x, y).take() {
                    let possible_new_x = (x + self.gravity_x).max(0).min(self.width() - 1);
                    let possible_new_y = (y + self.gravity_y).max(0).min(self.height() - 1);

                    let (new_x, new_y) =
                        if self.get_element(possible_new_x, possible_new_y).is_some() {
                            (x, y)
                        } else {
                            (possible_new_x, possible_new_y)
                        };

                    *self.get_element_mut(new_x, new_y) = Some(element);
                }
            }
        }
    }
}
