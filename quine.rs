fn main() {
    let src = "fn main() {\n    let src = \"?\";\n\n    for i in 0..src.len() {\n        if let Some(ch) = src.chars().nth(i) {\n            match ch as u8 {\n                63u8 => {\n                    for j in 0..src.len() {\n                        if let Some(ch) = src.chars().nth(j) {\n                            match ch {\n                                '\\n' => print!(\"\\\\n\"),\n                                '\"' => print!(\"\\\\\\\"\"),\n                                '\\\\' => print!(\"\\\\\\\\\"),\n                                _ => print!(\"{}\", ch),\n                            }\n                        }\n                    }\n                },\n                _ => print!(\"{}\", ch),\n            }\n        }\n    }\n}\n";

    for i in 0..src.len() {
        if let Some(ch) = src.chars().nth(i) {
            match ch as u8 {
                63u8 => {
                    for j in 0..src.len() {
                        if let Some(ch) = src.chars().nth(j) {
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
