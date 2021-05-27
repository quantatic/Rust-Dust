use std::convert::TryFrom;

#[derive(Debug)]
pub struct World {
    elements: Vec<Option<Element>>, // access as world[y * width + x]
    width: i32,
    height: i32,
}

#[derive(Clone, Copy, Debug)]
pub enum Element {
    Dust,
}

impl World {
    pub fn new(width: i32, height: i32) -> Self {
        let elements = vec![None; usize::try_from(width * height).unwrap()];
        Self {
            elements,
            width,
            height,
        }
    }

    fn element_idx(&self, x: i32, y: i32) -> usize {
        usize::try_from((y * self.width) + x).unwrap()
    }

    pub fn get_element(&self, x: i32, y: i32) -> &Option<Element> {
        let idx = self.element_idx(x, y);
        &self.elements[idx]
    }

    pub fn get_element_mut(&mut self, x: i32, y: i32) -> &mut Option<Element> {
        let idx = self.element_idx(x, y);
        &mut self.elements[idx]
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
                    let possible_new_y = (y - 1).max(0).min(self.height() - 1);

                    let possible_left_x = (x - 1).max(0);
                    let possible_right_x = (x + 1).min(self.width() - 1);

                    let (new_x, new_y) = if self.get_element(x, possible_new_y).is_none() {
                        (x, possible_new_y)
                    } else if self.get_element(possible_left_x, possible_new_y).is_none() {
                        (possible_left_x, possible_new_y)
                    } else if self.get_element(possible_right_x, possible_new_y).is_none() {
                        (possible_right_x, possible_new_y)
                    } else {
                        (x, y)
                    };

                    *self.get_element_mut(new_x, new_y) = Some(element);
                }
            }
        }
    }
}
