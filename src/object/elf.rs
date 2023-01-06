use goblin::{elf, Object};
use std::vec::Vec;
use std::collections::HashMap;
use crate::object::symbol::Symbol;

struct ELF<'a> {
    data_bytes: &'a Vec<u8>,
    object: elf::Elf<'a>,
}

impl<'a> ELF<'a> {
    /// Load object binary data from a &Vec<u8>.
    pub fn new(data_bytes: &'a Vec<u8>) -> Self {
        let elf = match Object::parse(&data_bytes).unwrap() {
            Object::Elf(elf) => elf,
            _ => unimplemented!(),
        };

        Self {
            data_bytes,
            object: elf
        }
    }

    /// Get symbols returned encapsulated in Vec.
    pub fn symbol_to_vec(&self) -> Vec<Symbol> {
        let mut result = Vec::<Symbol>::new();
        for sym in self.object.syms.iter() {
            result.push(Symbol::new(&self.object, &sym));
        }
        result
    }

    /// Get symbols returned encapsulated in HashMap.
    /// 
    /// The key is symbol's name, the value is symbol structure.
    pub fn symbol_to_map(&self) -> HashMap<String, Symbol> {
        let mut result = HashMap::<String, Symbol>::new();

        for sym in self.object.syms.iter() {
            let symbol = Symbol::new(&self.object, &sym);
            result.insert(symbol.name.to_string(), symbol);
        }

        result
    }

    pub fn get_symbol_by_name(&self, name: &str) -> Option<Symbol> {
        for sym in self.object.syms.iter() {
            if Symbol::name(&self.object, &sym) == name {
                return Some(Symbol::new(&self.object, &sym));
            }
        }
        None
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_symbol_to_vec() {
        let elf_bytes = include_bytes!("../../test/simple.amd64.elf").to_vec();
        let elf = ELF::new(&elf_bytes);
        for symbol in elf.symbol_to_vec() {
            assert!(symbol.name != "main" || (symbol.address, symbol.size) == (4393, 15));
            assert!(symbol.name != "a" || (symbol.address, symbol.size) == (4408, 11));
            assert!(symbol.name != "b" || (symbol.address, symbol.size) == (4419, 11));
            assert!(symbol.name != "c" || (symbol.address, symbol.size) == (4430, 11));
            println!("{:?}", symbol);
        }
    }

    #[test]
    fn test_symbol_to_map() {
        let elf_bytes = include_bytes!("../../test/simple.amd64.elf").to_vec();
        let elf = ELF::new(&elf_bytes);
        let symbol_map = elf.symbol_to_map();
        {
            let validate_sym = symbol_map.get("main").unwrap();
            assert!((validate_sym.address, validate_sym.size) == (4393, 15));
        }
        {
            let validate_sym = symbol_map.get("a").unwrap();
            assert!((validate_sym.address, validate_sym.size) == (4408, 11));
        }
        {
            let validate_sym = symbol_map.get("b").unwrap();
            assert!((validate_sym.address, validate_sym.size) == (4419, 11));
        }
        {
            let validate_sym = symbol_map.get("c").unwrap();
            assert!((validate_sym.address, validate_sym.size) == (4430, 11));
        }
    }

    #[test]
    fn test_get_symbol_by_name() {
        let elf_bytes = include_bytes!("../../test/simple.amd64.elf").to_vec();
        let elf = ELF::new(&elf_bytes);
        assert!(elf.get_symbol_by_name("a").is_some());
        assert!(elf.get_symbol_by_name("d").is_none());
    }
}