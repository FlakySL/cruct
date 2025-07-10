pub mod fields;
pub mod impl_block;

#[cfg(test)]
mod tests;

pub use fields::generate_field_initialization;
pub use impl_block::generate_impl_block;
