use std::{collections::HashMap};
use num_traits::{Float, AsPrimitive};

use crate::cell_list::*;


/// Classic cell list implementation for model based on 3D point representation (molecular dynamics etc.)
/// 
/// It contains generic [CellList] and a 3D points vector slice.
#[derive(Debug)]
pub struct CellList3DPoints<'a, T: Point3D>  {
    clist: CellList,
    origin: &'a [T]
}

impl<'a, T: Point3D> CellList3DPoints<'a, T> {
    /// Build cell list for 3D points from `origin` and keeps reference to the data inside. Require bounding box (simulation volume) `min` and `max` corner points. `cell_size` sets a size and determines the division of space into cells.
    pub fn build(
        origin: &'a [T],
        box_min_coord: T,
        box_max_coord: T,
        cell_side: T::Precision
    ) -> Self {
        let mut head = HashMap::new();
        let mut list = vec![0;origin.len() + 1];
        for (i,point) in origin.iter().enumerate() {
            let i = i + 1;
            let cell = {
                let nx = ((box_max_coord.x() - box_min_coord.x()) / cell_side).ceil();
                let ny = ((box_max_coord.y() - box_min_coord.y()) / cell_side).ceil();
                (  ((point.x()-box_min_coord.x()) / cell_side).floor())
                + (((point.y()-box_min_coord.y()) / cell_side).floor())*nx
                + (((point.z()-box_min_coord.z()) / cell_side).floor())*nx*ny
            }.as_();
            dbg!((&point.x(), point.y(), point.z(), &cell));
            let head_e = head.entry(cell).or_insert(0);
            list[i] = *head_e;
            *head_e = i;
        }
        Self{ clist: CellList { head, list } , origin }
    }

    /// Return iterator over points in a cell with given index, or [None] if the cell does not exist.
    pub fn iter_cell_points(&'a self, index: usize) -> Option<IterPoints<'a, T>> {
        if let Some(&pos) = self.clist.head.get(&index) {
            let cell_iter = CellItemsIter { clist: &self.clist, pos };
            Some(IterPoints { cell_iter, origin: self.origin } )
        } else {
            None
        }
    }

    /// Return iterator over cells in this structure.
    pub fn iter_cells(&'a self) -> IterCells<'a,T> {
        IterCells { clist: self, cells_iter: self.clist.head.iter() }
    }
}

pub struct IterPoints<'a, T: Point3D> {
    cell_iter: CellItemsIter<'a>,
    origin:  &'a [T]
}

impl<'a, T: Point3D> Iterator for IterPoints<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cell_iter.next().map(|id| &self.origin[id])
    }
}

pub struct IterCells<'a, T: Point3D> {
    clist: &'a CellList3DPoints<'a, T>,
    cells_iter: std::collections::hash_map::Iter<'a, usize, usize>
}

impl<'a, T: Point3D> Iterator for IterCells<'a,T> {
    type Item = (usize, IterPoints<'a,T>);

    fn next(&mut self) -> Option<Self::Item> {
        self.cells_iter.next().map( 
            |(&cell,_)|  (cell, self.clist.iter_cell_points(cell).unwrap())
        )
    }
}

/// [Point3D] trait enable generalization over float precision and points representation ([x,y,z] or (x,y,z) or user defined structure).
pub trait Point3D {
    type Precision : Float + AsPrimitive<usize> + std::fmt::Debug;

    fn x(&self) -> Self::Precision;
    fn y(&self) -> Self::Precision;
    fn z(&self) -> Self::Precision;
}

impl<T: Float + AsPrimitive<usize> + std::fmt::Debug> Point3D for [T; 3] {
    type Precision = T;

    #[inline]
    fn x(&self) -> Self::Precision {
        self[0]
    }
    #[inline]
    fn y(&self) -> Self::Precision {
        self[1]
    }
    #[inline]
    fn z(&self) -> Self::Precision {
        self[2]
    }
}

impl<T: Float + AsPrimitive<usize> + std::fmt::Debug> Point3D for (T,T,T) {
    type Precision = T;

    #[inline]
    fn x(&self) -> Self::Precision {
        self.0
    }
    #[inline]
    fn y(&self) -> Self::Precision {
        self.1
    }
    #[inline]
    fn z(&self) -> Self::Precision {
        self.2
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::prelude::*;
    use std::{fs::File, io::{BufWriter, Write}, collections::{HashSet, BTreeSet}};

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
        for p in cl.iter_cell_points(2).unwrap() {
            dbg!(&p);
        }
    }

    #[test]
    fn test_cell_list_print_full_iter() {
        let v = &[[2.0 as f32, 2.0, 1.0], 
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
            4.0f32
        );

        for (id, cell) in cl.iter_cells() {
            println!("========{}==========", id);
            for point in cell {
                dbg!(point);
            }
        }
    }

    #[test]
    fn test_random_xyz_visualization() {
        let mut v = Vec::new();
        let min = -10.0;
        let max = 10.0;
        
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            v.push([rng.gen_range(min..=max),rng.gen_range(min..=max),rng.gen_range(min..=max)]);
        }
        let cl = CellList3DPoints::build(
            &v,
            [min,min,min],
            [max,max,max],
            5.0f64
        );

        let mut out = String::new();
        out.push_str(&format!("{}\n\n",v.len()));
        for (id, cell) in cl.iter_cells() {
            for point in cell {
                out.push_str(&format!("{}\t{}\t{}\t{}\n", id, point[0], point[1], point[2]));
            }
        }
        let mut f = BufWriter::new(File::create("./visualization.xyz").expect("Unable to create file"));
        f.write_all(out.as_bytes()).expect("Unable to write data");
    }
}