pub mod latex;

use super::states::outer_structure::meta::Meta;

pub trait Writer {
    fn print_begin(&mut self);
    fn print_end(&mut self);
    fn print_meta(&mut self, meta: Meta);
    fn print_begin_document(&mut self);
    fn print_end_document(&mut self);
}
