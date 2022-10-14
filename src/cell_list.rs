use std::collections::HashMap;

const USIZE_BITS: usize = core::mem::size_of::<usize>() * 8;
const MAX_CAPACITY: usize = 1 << (USIZE_BITS-1);

#[derive(Debug, Clone)]
pub struct CellList {
    pub(crate) head: HashMap<usize,usize>,
    pub(crate) list: Vec<usize>,
}

impl CellList {
    pub fn new() -> Self {
        Self { head: HashMap::new(), list: Vec::new() }
    }

/// Append a value to a correspondng cell.
    pub fn push(&mut self, cell: usize, value: usize) {
        let value = value + 1;
        let head_e = self.head.entry(cell).or_insert(0);
        if let Some(_) = (value+1).checked_sub(self.list.len()) {
            self.list.resize(value+1, 0);
        }
        self.list[value] = *head_e;
        *head_e = value;
    }

    pub fn iter_cell_items(&self, cell_index: usize) -> Option<CellItemsIter<'_>> {
        if let Some(&pos) = self.head.get(&cell_index) {
            Some(CellItemsIter { clist: &self, pos } )
        } else {
            None
        }
    }
}

impl Default for CellList {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CellItemsIter<'a> {
    pub(crate) clist: &'a CellList,
    pub(crate) pos: usize
}

impl<'a> Iterator for CellItemsIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        if pos <= 0 {
            return None;
        }
        self.pos = self.clist.list[pos];
        Some(pos-1)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cell_list_get_unwrap() {
        let v = vec![9,9,8,1,1,1,2,2,2,3,4,5];
        let mut cl = CellList::new();
        for (i,x) in v.iter().enumerate() {
            cl.push(x/2usize, i);
        }
        dbg!(&cl);
        for i in cl.iter_cell_items(1).unwrap() {
            dbg!((&i, v[i]));
        }
    }

    #[test]
    fn test_saving_inc() {
        let cell = 1;
        let mut cl = CellList::new();
        let r = 0..100;
        for i in r.clone() {
            cl.push(cell, i);
        }
        let mut r = r;
        for val in cl.iter_cell_items(cell).unwrap() {
            assert_eq!(r.next_back().unwrap(), val);
        }
    }

    #[test]
    fn test_saving_dec() {
        let mut cl = CellList::new();
        let r = 0..100;
        for i in r.clone().rev() {
            cl.push(i/50, i);
        }
        let mut r = 50..100;
        for val in cl.iter_cell_items(1).unwrap() {
            assert_eq!(r.next().unwrap(), val);
        }
    }
}

