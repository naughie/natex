pub mod latex;

use super::states::outer_structure::meta::Meta;

pub trait Writer {
    fn print_meta(&mut self, meta: Meta);
}
