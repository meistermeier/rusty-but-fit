use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use calamine::{Data, DataType, open_workbook, Reader, ToCellDeserializer, Xlsx};
use convert_case::{Case, Casing};

fn main() {
    let mut workbook: Xlsx<_> = open_workbook("Profile.xlsx").unwrap();
    let mut first_row = true;
    let mut buffer = String::new();
    std::fs::remove_file("types.rs").unwrap();
    let mut target = File::create("types.rs").unwrap();
    target.write("use std::fmt;\nuse std::fmt::{Display, Formatter};\n".as_bytes()).unwrap();
    let mut prev_value = Data::Empty;
    let mut type_numbers = HashMap::new();
    let mut mesg_num_parsing = false;
    if let Ok(r) = workbook.worksheet_range("Types") {
        for row in r.rows() {
            if first_row {
                first_row = false;
                continue;
            }
            if ToCellDeserializer::is_empty(&row[0]) {
                if !row[3].eq(&prev_value) {
                    if mesg_num_parsing {
                        type_numbers.insert(row[2].as_string().unwrap(), row[3].as_string().unwrap());
                    }
                    buffer.push_str(format!("\t\t{} = {},\n", make_nice(row[2].to_string()), row[3]).as_str())
                }
                prev_value = row[3].clone();
            } else {
                if !buffer.is_empty() {
                    buffer.push_str("\t}\n}\n");
                    target.write(buffer.as_bytes()).unwrap();
                    buffer.clear();
                    prev_value = Data::Empty;
                    mesg_num_parsing = false;
                }
                buffer.push_str("crate::key_value_enum! {\n");
                buffer.push_str("\tpub enum ");
                if "mesg_num".to_string().eq(&row[0].as_string().unwrap()) {
                    mesg_num_parsing = true;
                }
                buffer.push_str(make_nice(row[0].as_string().unwrap()).as_str());
                buffer.push_str(" { \n");
            }
        }
        if !buffer.is_empty() {
            buffer.push_str("\t}\n}\n");
            target.write(buffer.as_bytes()).unwrap();
            buffer.clear();
            prev_value = Data::Empty;
            mesg_num_parsing = false;
        }
    }
    first_row = true;
    buffer.clear();
    let mut buffer2 = String::new();
    let mut current_mesg_num = "";

    if let Ok(r) = workbook.worksheet_range("Messages") {
        for messages_row in r.rows() {
            if first_row {
                first_row = false;
                continue;
            }
            if ToCellDeserializer::is_empty(&messages_row[0]) {
                if !ToCellDeserializer::is_empty(&messages_row[2]) {
                    let string = messages_row[1].to_string();
                    if !string.is_empty() {
                        if primitive_type(messages_row[3].as_string().unwrap()) {
                            buffer.push_str(format!("{},{},\"{}\"\n", current_mesg_num, string, messages_row[2].to_string()).as_str());
                        } else {
                            buffer2.push_str(format!("{},{},\"{}\",{}\n", current_mesg_num, string, messages_row[2].to_string(), make_nice(messages_row[3].as_string().unwrap())).as_str());
                        }
                    }
                }
            } else {
                // if !buffer.is_empty() {
                //     print!("{}", buffer);
                //     buffer.clear();
                // }
                // create entry and start over
                current_mesg_num = type_numbers.get(&messages_row[0].as_string().unwrap()).unwrap().as_str();
            }
        }
        println!("{}", "crate::expand_fields! {");
        println!("{}", buffer);
        println!("{}", "}");
        println!("{}", "crate::expand_fields! {");
        println!("{}", buffer2);
        println!("{}", "}");
    }
}

fn primitive_type(input: String) -> bool {
    match input.to_lowercase().as_str() {
        "uint64" => true,
        "uint32" => true,
        "uint16" => true,
        "uint8" => true,
        "uint64z" => true,
        "uint32z" => true,
        "uint16z" => true,
        "uint8z" => true,
        "sint8" => true,
        "sint16" => true,
        "sint32" => true,
        "sint64" => true,
        "string" => true,
        "byte" => true,
        "float32" => true,
        "float64" => true,
        "bool" => true,
        _ => false,
    }
}

fn make_nice(input: String) -> String {
    match input.to_lowercase().as_str() {
        "uint64" => "NumberValueU64".to_string(),
        "uint32" => "NumberValueU32".to_string(),
        "uint16" => "NumberValueU16".to_string(),
        "uint8" => "NumberValueU8".to_string(),
        "uint64z" => "NumberValueU64".to_string(),
        "uint32z" => "NumberValueU32".to_string(),
        "uint16z" => "NumberValueU16".to_string(),
        "uint8z" => "NumberValueU8".to_string(),
        "sint8" => "NumberValueS8".to_string(),
        "sint16" => "NumberValueS16".to_string(),
        "sint32" => "NumberValueS32".to_string(),
        "sint64" => "NumberValueS64".to_string(),
        "string" => "StringValue".to_string(),
        // "bool" => "Value".to_string(),
        "byte" => "NumberValueU8".to_string(),
        "float32" => "NumberValueU64".to_string(),
        "float64" => "NumberValueU64".to_string(),
        // "product" => "GarminProduct".to_string(),
        _ => input
            .replace("1", "one")
            .replace("2", "two")
            .replace("3", "three")
            .replace("4", "four")
            .replace("5", "five")
            .replace("6", "six")
            .replace("7", "seven")
            .replace("8", "eight")
            .replace("9", "nine")
            .to_case(Case::Pascal)
    }
}
