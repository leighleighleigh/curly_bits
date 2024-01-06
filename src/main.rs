extern crate curly_bits;
use curly_bits::load_curly_bits;

load_curly_bits!("tests/simple.txt");

fn main() {
    let mut data : SimpleTemplate = SimpleTemplate::default();
    data.greeting = "Hey".to_string();
    data.name = "Dude".to_string();

    println!("#### template ({})\n{}\n", data.template_file(), data.template());
    println!("#### struct\n{:#?}\n", data);
    println!("#### result\n{}\n", data);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        load_curly_bits!("tests/simple.txt");

        let mut data : SimpleTemplate = SimpleTemplate::default();
        data.greeting = "Hello".to_string();
        data.name = "World".to_string();

        assert!(format!("{}",data) == "Hello, World".to_string())
    }

    #[test]
    fn test_simple_default() {
        load_curly_bits!("tests/simple.txt");

        let data : SimpleTemplate = SimpleTemplate::default();

        assert!(format!("{}",data) == "{{greeting}}, {{name}}".to_string())
    }

    #[test]
    fn test_huge() {
        // this shouldn't take long
        load_curly_bits!("tests/huge.txt");

        for _ in 0..100 {
            let data : HugeTemplate = HugeTemplate::default();

            println!("### template ({})\n{}\n", data.template_file(), data.template());
            println!("### struct\n{:#?}\n", data);
            println!("### result\n{}\n", data);
        }
    }
}
