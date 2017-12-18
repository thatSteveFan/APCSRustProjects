#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub mod main
{
    use std::io;
    pub fn run()
    {
        

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
                
                println!("reversed: {}", reverse_string(&input))
            }
            Err(error) => println!("error: {}", error),
        }
    }

    use std::string::String;
    fn reverse_string(rev: &str) -> String
    {
        let mut returning = String::new();
        for char in rev.chars().rev()
        {
            returning.push(char);
        }
        returning
    }
}
