//! Cell lists or cell linked-lists are classic data structure used in molecular dynamics to reduce the computational cost of finding all interacting pairs within a given cut-off from O(N^2) to O(N). It can be used together with Verlet neighbor lists.
//!
//! [cell_list] is a generic implementation of the structure that does not store a any source data and does not know anything about the 3D nature of the problem.
//!
//! [cell_list_3d_points] includes a reference to the points array and require simulation volume parameters to build the structure.
//! 
//!  # Refs
//! Allen; Tildesley. Computer Simulation of Liquids.

pub mod cell_list;
pub mod cell_list_3d_points;
