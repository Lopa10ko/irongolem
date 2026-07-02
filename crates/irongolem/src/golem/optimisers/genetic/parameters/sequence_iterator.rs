use std::cell::RefCell;
use std::collections::HashMap;

pub struct SequenceIterator {
    sequence_func: fn(usize) -> usize,
    archive: RefCell<HashMap<isize, usize>>,
    index: RefCell<isize>,
    max_sequence_value: Option<usize>,
    min_sequence_value: Option<usize>,
}

impl SequenceIterator {
    pub fn new(
        sequence_func: fn(usize) -> usize,
        start_value: Option<usize>,
        max_sequence_value: Option<usize>,
        min_sequence_value: Option<usize>,
    ) -> Self {
        let index = if let Some(start) = start_value {
            Self::get_sequence_index(sequence_func, start) as isize - 1
        } else {
            -1
        };
        Self {
            sequence_func,
            archive: RefCell::new(HashMap::new()),
            index: RefCell::new(index),
            max_sequence_value,
            min_sequence_value,
        }
    }

    fn sequence_item_calculation(&self, index: isize) -> usize {
        let mut archive = self.archive.borrow_mut();
        if let Some(&value) = archive.get(&index) {
            return value;
        }
        let result = (self.sequence_func)(index.max(0) as usize);
        archive.insert(index, result);
        result
    }

    fn get_sequence_index(sequence_func: fn(usize) -> usize, value: usize) -> usize {
        let mut number = 0usize;
        while sequence_func(number) < value {
            number += 1;
        }
        number
    }

    pub fn has_prev(&self) -> bool {
        let index = *self.index.borrow();
        if index > 0 {
            if let Some(min_val) = self.min_sequence_value {
                self.sequence_item_calculation(index - 1) >= min_val
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn has_next(&self) -> bool {
        let index = *self.index.borrow();
        if let Some(max_val) = self.max_sequence_value {
            self.sequence_item_calculation(index + 1) <= max_val
        } else {
            true
        }
    }

    pub fn next_value(&self) -> usize {
        let mut index = self.index.borrow_mut();
        *index += 1;
        if let Some(min_val) = self.min_sequence_value {
            if self.sequence_item_calculation(*index) < min_val {
                *index = Self::get_sequence_index(self.sequence_func, min_val) as isize;
            }
        }
        self.sequence_item_calculation(*index)
    }

    pub fn prev_value(&self) -> Option<usize> {
        let mut index = self.index.borrow_mut();
        *index -= 1;
        if *index < 0 {
            return None;
        }
        Some(self.sequence_item_calculation(*index))
    }

    pub fn current(&self) -> usize {
        self.sequence_item_calculation(*self.index.borrow())
    }
}

pub fn fibonacci_sequence(n: usize) -> usize {
    let mut a = 0usize;
    let mut b = 1usize;
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a
}
