use std::collections::HashMap;

#[derive(Debug)]
pub struct CellList<'a, T> {
    head: HashMap<usize,usize>,
    list: Vec<usize>,
    origin: &'a [T]
}

impl<'a, T> CellList<'a, T> {
    pub fn build(origin: &'a [T], cell_num: impl Fn(&T) -> usize) -> Self {
        let mut head = HashMap::new();
        let mut list = vec![0;origin.len()+1];
        for (i,point) in origin.iter().enumerate() {
            let i = i+1;
            let cell = cell_num(point);
            let head_e = head.entry(cell).or_insert(0);
            list[i] = *head_e;
            *head_e = i;
        }
        Self { head, list, origin }
    }

    pub fn get(&'a self, index: usize) -> Option<CellIter<'a,T>> {
        if let Some(&pos) = self.head.get(&index) {
            Some(CellIter { clist: &self, pos })
        } else {
            None
        }
    }
}

impl<'a> CellList<'a, (f64, f64)> {
    pub fn build_by_geometry(
        origin: &'a [(f64,f64)], 
        cell_min_coord: (f64,f64),
        cell_max_coord: (f64,f64),
        step: f64
    ) -> Self {
        let m = 1.0 / step;
        let nx = ((cell_max_coord.0 - cell_min_coord.0) * m) as usize;
        let by_dist = |(x, y) : &(f64,f64)| (((x+0.5)*m) as usize) + (((y+0.5)*m) as usize)*nx ;
        Self::build(origin, by_dist )
    }
}

pub struct CellIter<'a, T> {
    clist: &'a CellList<'a, T>,
    pos: usize
}

impl<'a,T> Iterator for CellIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        if pos == 0 {
            return None;
        }
        self.pos = self.clist.list[pos];
        self.clist.origin.get(pos-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unwrap() {
        let v = vec![1,1,1,2,2,2,4,4,4,5];
        let cl = CellList::build(&v, |&x| x);
        dbg!(&cl);
        for i in cl.get(4).unwrap() {
            dbg!(&i);
        }
    }

    #[test]
    fn test_points_by_distance() {
        let v = &[(2.0 as f64, 2.0 as f64), (2.0, 8.0), (5.0, 5.0), (5.0, 5.0), 
        (6.0, 3.0), (6.0, 7.0), (7.0, 4.0), (7.0, 9.0)];
        let (mut minx, mut miny)  = v[0];
        let (mut maxx, mut maxy) = v[0];
        for i in v {
            minx = minx.min(i.0);
            maxx = maxx.max(i.0);
            miny = miny.min(i.1);
            maxy = maxy.max(i.1);
        }
        let cl = CellList::build_by_geometry(
            v,
            (minx, miny),
            (maxx, maxy),
            5.0f64
        );
        dbg!(&cl);
        for i in cl.get(2).unwrap() {
            dbg!(&i);
        }
    }
}
