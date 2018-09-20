extern crate num_traits as num;
#[macro_use]
extern crate approx;
extern crate alga;
extern crate nalgebra as na;

pub use aliases::*;
pub use constructors::*;
pub use geometric::*;
pub use matrix::*;
pub use traits::*;
pub use trigonometric::*;
pub use vector_relational::*;
pub use exponential::*;

pub use ext_vector_relational::*;

pub use gtx_component_wise::*;

mod aliases;
pub mod constructors;
mod common;
pub mod matrix;
pub mod geometric;
mod traits;
pub mod trigonometric;
pub mod vector_relational;
pub mod exponential;
pub mod integer;
pub mod packing;
pub mod ext_matrix_clip_space;
pub mod ext_matrix_projection;
pub mod ext_matrix_relationnal;
pub mod ext_matrix_transform;
pub mod ext_quaternion_common;
pub mod ext_quaternion_geometric;
pub mod ext_quaternion_transform;
pub mod ext_quaternion_trigonometric;
pub mod ext_quaternion_relational;
pub mod ext_scalar_common;
pub mod ext_scalar_constants;
pub mod ext_vector_common;
pub mod ext_vector_relational;
//pub mod gtc_bitfield;
pub mod gtc_constants;
pub mod gtc_epsilon;
//pub mod gtc_integer;
pub mod gtc_matrix_access;
pub mod gtc_matrix_inverse;
//pub mod gtc_packing;
pub mod gtc_quaternion;
//pub mod gtc_reciprocal;
//pub mod gtc_round;
pub mod gtc_type_ptr;
//pub mod gtc_ulp;

pub mod gtx_component_wise;
//pub mod gtx_euler_angles;
pub mod gtx_exterior_product;
pub mod gtx_handed_coordinate_space;
pub mod gtx_matrix_cross_product;
pub mod gtx_matrix_operation;
pub mod gtx_norm;
pub mod gtx_normal;
pub mod gtx_normalize_dot;
//pub mod gtx_quaternion;
pub mod gtx_rotate_normalized_axis;
//pub mod gtx_rotate_vector;
pub mod gtx_transform;
//pub mod gtx_transform2;
//pub mod gtx_transform2d;
//pub mod gtx_vector_angle;
pub mod gtx_vector_query;