fn print_source_code(source: &str) {
    for i in 0..source.len() {
        if let Some(ch) = source.chars().nth(i) {
            match ch as u8 {
                63u8 => {
                    for j in 0..source.len() {
                        if let Some(ch) = source.chars().nth(j) {
                            match ch {
                                '\n' => print!("\\n"),
                                '"' => print!("\\\""),
                                '\\' => print!("\\\\"),
                                _ => print!("{}", ch),
                            }
                        }
                    }
                },
                _ => print!("{}", ch),
            }
        }
    }
}

fn main() {
    let src = "fn print_source_code(source: &str) {\n    for i in 0..source.len() {\n        if let Some(ch) = source.chars().nth(i) {\n            match ch as u8 {\n                63u8 => {\n                    for j in 0..source.len() {\n                        if let Some(ch) = source.chars().nth(j) {\n                            match ch {\n                                '\\n' => print!(\"\\\\n\"),\n                                '\"' => print!(\"\\\\\\\"\"),\n                                '\\\\' => print!(\"\\\\\\\\\"),\n                                _ => print!(\"{}\", ch),\n                            }\n                        }\n                    }\n                },\n                _ => print!(\"{}\", ch),\n            }\n        }\n    }\n}\n\nfn main() {\n    let src = \"?\";\n\n    print_source_code(src);\n}\n";

    print_source_code(src);
}
