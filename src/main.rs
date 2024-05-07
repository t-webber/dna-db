#![allow(dead_code)]

use std::ffi::CString;
use std::fmt::Write;

struct FileHeader {
    nb_blocks: u64,
    block_sizes: Vec<u64>,
    types: Vec<Type>,
    structs: Vec<Struct>,
    enums: Vec<Enums>,
}

#[derive(Debug)]
#[repr(C)]
enum TypeType {
    Raw,
    Struct(CString),
    Enum(CString),
}

#[derive(Debug)]
#[repr(C)]
struct Type {
    name: CString,
    size: usize,
    type_: TypeType,
}

#[derive(Debug)]
#[repr(C)]
struct StructField {
    name: CString,
    type_: Vec<u8>,
}

/// Type User
/// Type MRI
#[derive(Debug)]
#[repr(C)]
struct Struct {
    name: CString,
    fields: Vec<StructField>,
}

#[derive(Debug)]
#[repr(C)]
struct EnumField {
    name: CString,
    type_: Vec<u8>,
}

/// Type Role Type
#[derive(Debug)]
#[repr(C)]
struct Enums {
    name: CString,
    fields: Vec<EnumField>,
}

enum Legumes {
    Courgettes(u8),
    Carottes { a: u64, b: u64, c: Vec<String> },
}

#[repr(C)]
enum A {
    C(()),
    B(String),
}

struct User {
    firstname: String,
    role: Role,
}

struct Role {
    name: String,
    email: String,
}

/// {
///     name: "User",
///     fields: [
///         {
///             name: "firstname",
///             type:
///         }
///     ]
/// }

/// All instances of type User
#[repr(C)]
struct DataBlock {
    name: CString,
    data: Vec<Item>,
}

/// An instance of a type = 1 user
#[repr(C)]
struct Item {
    name: CString,
    data: Vec<u8>,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Toto {
//     a: u32,
//     b: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Titi {
//     a: Vec<Toto>,
// }

fn get_header(fileheader: FileHeader) -> String {
    let mut header = String::new();
    write!(header, "{:064b}", fileheader.nb_blocks).unwrap();
    for block_size in fileheader.block_sizes {
        write!(header, "{:064b}", block_size).unwrap();
    }
    header
}

const DB_PATH: &str = "./dev.db";

fn main() {
    let header = get_header(FileHeader {
        nb_blocks: 13792,
        block_sizes: vec![2292, 2032],
        types: vec![],
        structs: vec![],
        enums: vec![],
    });
    std::fs::write(DB_PATH, header.as_bytes()).unwrap()
}

// mod test {
//     #[test]
//     fn
// }
