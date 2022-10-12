use std::collections::HashMap;
use crate::cell_list::*;

#[derive(Debug)]
pub struct CellList3DPoints<'a, T> where T : AsRef<[f64]> {
    clist: CellList,
    origin: &'a [T]
}

impl<'a, T: AsRef<[f64]>> CellList3DPoints<'a, T> {
    pub fn build(
        origin: &'a [T],
        cell_min_coord: T,
        cell_max_coord: T,
        step: f64
    ) -> Self {
        let cell_min_coord = cell_min_coord.as_ref();
        let cell_max_coord = cell_max_coord.as_ref();
        let mut head = HashMap::new();
        let mut list = vec![0;origin.len()+1];
        for (i,point) in origin.iter().enumerate() {
            let i = i+1;
            let cell = {
                let m = 1.0 / step;
                let nx = ((cell_max_coord[0] - cell_min_coord[0]) * m) as usize;
                let ny = ((cell_max_coord[1] - cell_min_coord[1]) * m) as usize;
                (((point.as_ref()[0]+cell_min_coord[0]+0.5)*m) as usize)
                + (((point.as_ref()[1]+cell_min_coord[1]+0.5)*m) as usize)*nx
                + (((point.as_ref()[2]+cell_min_coord[2]+0.5)*m) as usize)*nx*ny
            };
            let head_e = head.entry(cell).or_insert(0);
            list[i] = *head_e;
            *head_e = i;
        }
        Self{ clist: CellList { head, list } , origin }
    }

    pub fn get_cell_points_iter(&'a self, index: usize) -> Option<PointsIter<'a, T>> {
        if let Some(&pos) = self.clist.head.get(&index) {
            let cell_iter = CellItemsIter { clist: &self.clist, pos };
            Some(PointsIter { cell_iter, origin: self.origin } )
        } else {
            None
        }
    }
}

pub struct PointsIter<'a, T: AsRef<[f64]>> {
    cell_iter: CellItemsIter<'a>,
    origin:  &'a [T]
}

impl<'a, T: AsRef<[f64]>> Iterator for PointsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cell_iter.next().map(|id| &self.origin[id])
    }
}

pub struct CellsIter<'a> {
    clist: &'a CellList,
    cells_iter: std::collections::hash_map::Iter<'a, usize, usize>
}

impl<'a> Iterator for CellsIter<'a> {
    type Item = (usize,CellItemsIter<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.cells_iter.next().map( 
            |(&cell,&start)|  (cell, CellItemsIter { clist: self.clist, pos: start })
        )
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_points_by_distance() {
        let v = &[[2.0 as f64, 2.0, 1.0], 
        [2.0, 8.0, 1.0], [5.0, 5.0, 1.0], [5.0, 5.0, -1.0], 
        [6.0, 3.0, -1.0], [6.0, 7.0, 0.0], [7.0, 4.0, 0.0],
        [7.0, 9.0, 0.0]];
        let mut min  = v[0];
        let mut max = v[0];
        for p in v {
            for i in 0..3 {
                min[i] = min[i].min(p[i]);
                max[i] = max[i].max(p[i]);
            }
        }
        let cl = CellList3DPoints::build(
            v,
            min,
            max,
            4.0f64
        );
        dbg!(&cl);
        for p in cl.get_cell_points_iter(2).unwrap() {
            dbg!(&p);
        }

        // for cell in cl.get_cells_iter(index) {
        //     for point in cell {
        //         dbg!(point);
        //     }
        // }
    }
}