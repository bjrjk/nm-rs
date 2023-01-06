use goblin::elf::{Elf, sym::*};

/// Struct Symbol contains properties of object file's symbols.
/// 
/// ## Range
/// - Binding: LOCAL, GLOBAL, WEAK, ...
/// - Typ: NOTYPE, OBJECT, FUNC, SECTION, FILE, COMMON, TLS, ...
/// 
#[derive(Debug)]
pub struct Symbol<'a> {
    /// Symbol's name
    pub name: &'a str,
    /// Symbol's binding
    pub binding: &'static str,
    /// Symbol's type
    pub typ: &'static str,
    /// Symbol's visibility
    pub visibility: &'static str,
    /// Symbol's virtual address
    pub address: u64,
    /// Symbol's size
    pub size: u64,
}

impl<'a> Symbol<'a> {
    /// Construct a Symbol using given Elf and Sym.
    pub fn new<'b> (elf: &'a Elf, sym: &'b Sym) -> Self {
        Symbol {
            name: Self::name(elf, sym),
            binding: bind_to_str(sym.st_bind()),
            typ: type_to_str(sym.st_type()),
            visibility: visibility_to_str(sym.st_visibility()),
            address: sym.st_value,
            size: sym.st_size,
        }
    }

    pub fn name<'b> (elf: &'b Elf, sym: &Sym) -> &'b str {
        elf.strtab.get_at(sym.st_name).unwrap()
    }
}