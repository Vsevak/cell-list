use std::collections::HashMap;

#[derive(Debug)]
pub struct CellList {
    pub(crate) head: HashMap<usize,usize>,
    pub(crate) list: Vec<usize>,
}

impl CellList {
    pub fn new() -> Self {
        Self { head: HashMap::new(), list: Vec::new() }
    }

    pub fn push(&mut self, cell: usize, value: usize) {
        let value = value + 1;
        let head_e = self.head.entry(cell).or_insert(0);
        if let Some(delta) = (value+1).checked_sub(self.list.len()) {
            self.list.resize(value+1, 0);
        }
        self.list[value] = *head_e;
        *head_e = value;
    }

    pub fn get_cell_items_iter(&self, cell_index: usize) -> Option<CellItemsIter<'_>> {
        if let Some(&pos) = self.head.get(&cell_index) {
            Some(CellItemsIter { clist: &self, pos } )
        } else {
            None
        }
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
        for i in cl.get_cell_items_iter(1).unwrap() {
            dbg!((&i, v[i]));
        }
    }
}

