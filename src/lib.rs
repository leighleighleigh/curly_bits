// load_curly_bits macro - produces a simple String-field struct of the unique curly-brace-delimited fields,
// in the provided file.
extern crate proc_macro;
extern crate syn;
extern crate quote;

// #[macro_use]
use quote::quote;
use proc_macro::TokenStream;
use convert_case::{Case, Casing};
use std::path::PathBuf;
// use std::str::FromStr;
use std::collections::HashMap;
use std::fs;


#[proc_macro]
pub fn load_curly_bits(input: TokenStream) -> TokenStream {
    // This takes a path to a file,
    // and generates a rust struct from it. 
    // The name of the struct is taken frome the filename (in pascal case) + "Template" suffix.

    // Get the filename from the macro
    let input = input.to_string();
    let input = input.trim_matches('"');

    // get filename stem, which is used for the struct name
    let filename = PathBuf::from(input);
    let mut struct_name : String = filename.file_stem().unwrap().to_str().unwrap().to_string();
    // convert to camel case
    struct_name = struct_name.to_case(Case::Pascal);
    struct_name = format!("{}Template", struct_name);

    // iterate over the file, reading it, and extracting the curly-brace-delimited fields.
    let mut curly_bits : HashMap<String,usize> = HashMap::new();

    // Read to string in a line-by-line fashion
    let file_str = fs::read_to_string(input).expect("Failed to read file");

    let re = regex::Regex::new(r"\{\{.*?\}\}").unwrap();
    let inner_re = regex::Regex::new(r"\{\{(?P<field>.*?)\}\}").unwrap();

    for (i,line) in file_str.lines().enumerate() {
        // println!("line: {}",line);
        for cap in re.captures_iter(line) {
            // extact the field name, between the {{ and }}.
            // use another regex to extract the field name.
            for inner_cap in inner_re.captures_iter(&cap[0]) {
                // println!("inner_cap: {:?}",inner_cap);
                // println!("field: {}", &inner_cap["field"]);
                curly_bits.insert(inner_cap["field"].to_string(), i);
            }
        }
    }

    // sort the hashmap in ascending line order, and store the resulting sorted field names in a vector.
    let mut fields_lines : Vec<(String,usize)> = curly_bits.into_iter().collect();
    fields_lines.sort_by(|a, b| a.1.cmp(&b.1));
    let fields : Vec<String> = fields_lines.into_iter().map(|(k,_)| k).collect();


    let new_struct = syn::Ident::new(&struct_name, proc_macro2::Span::call_site());
    let struct_fields = fields.iter().map(|f| {
        let field_name = syn::Ident::new(&f, proc_macro2::Span::call_site());
        let field_type_str = "String".to_string();
        let field_type = syn::Ident::new(&field_type_str, proc_macro2::Span::call_site());

        quote! {
            // add the field unquoted
            #field_name : #field_type,
        }
    });


    // generate code to resolve a field name to a value at runtime
    let fields_runtime_map = fields.iter().map(|f| {
        let field_name = syn::Ident::new(&f, proc_macro2::Span::call_site());
        quote! {
            // add the field unquoted
            fields.insert(stringify!(#field_name).to_string(),self.#field_name.to_string());
        }
    });


    // default value for the struct is just the template string as it was, untouched.
    let default_struct_fields = fields.iter().map(|f| {
        let field_name = syn::Ident::new(&f, proc_macro2::Span::call_site());

        quote! {
            // add the field unquoted
            #field_name : format!("{{{{{}}}}}",stringify!(#field_name)),
        }
    });

    // assemble the pieces into the final struct definition.
    let gen = quote! {
        #[derive(Debug,Clone)]
        pub struct #new_struct {
            #(#struct_fields)*
        }

        impl #new_struct {
            // return a hashmap of field names and values.
            pub fn fields(&self) -> HashMap<String,String> {
                // returns field names and values from self, at runtime.
                let mut fields : HashMap<String,String> = HashMap::new();

                #(#fields_runtime_map)*

                fields
            }

            // returns filepath to the template file.
            pub fn template_file(&self) -> String {
                #input.to_string()
            }

            // return the original template data, as a string.
            pub fn template(&self) -> String {
                // the original template data, read to string
                let file_str = fs::read_to_string(self.template_file()).expect("Failed to read file");
                file_str
            }

            // render the template, replacing the field names with the values.
            pub fn render(&self) -> String {
                let mut template_str : String = self.template();
                // iterate over the self.fields hashmap, which contains the field names and latest runtime values.
                for (field,value) in self.fields() {
                    // replace the field name with the value.
                    template_str = template_str.replace(&format!("{{{{{}}}}}",field),&value);
                }
                template_str
            }
        }

        // impliment Display for the struct, which returns the rendered template.
        impl std::fmt::Display for #new_struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.render())
            }
        }

        impl Default for #new_struct {
            fn default() -> Self {
                #new_struct {
                    #(#default_struct_fields)*
                }
            }
        }

    };

    gen.into()
}
