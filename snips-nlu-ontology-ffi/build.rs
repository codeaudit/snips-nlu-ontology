extern crate cbindgen;

use std::env;

use cbindgen::Language;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let result = cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .with_include_guard("LIB_SNIPS_NLU_ONTOLOGY_H_")
        .with_autogen_warning(
            "/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */",
        )
        .with_documentation(false)
        .generate();

    match result {
        Ok(generator) => {
            generator.write_to_file("platforms/snips-nlu-ontology-c/libsnips_nlu_ontology.h");
        }
        Err(e) => {
            eprintln!("Unable to generate bindings. Reason: {}", e);
        }
    }
}
