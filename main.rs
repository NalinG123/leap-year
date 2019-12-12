fn main()
{
    use std::io::{stdin,stdout,Write};

    let mut input_text = String::new();
    print!("Please enter some number: ");
    let _=stdout().flush();
    stdin()

        .read_line(&mut input_text)

        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let mut n = 0;
    match trimmed.parse::<u64>() {
        Ok(i) => {n=i; println!("your integer input: {}", i)},

        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    println!("copied input {} and more {}",n, n);
    find_leapyear(n);
}

    fn find_leapyear(g: u64) -> bool
    {
       let  div = g%4;
       let  div1 = g%400;
       let mut val_leapyear = false;
        if div == 0 && div1 == 0
             {
            println!("it is a leap year");
            val_leapyear = true;
        }
        else 
        {
            println!("it is not a leap year");
            
        }
        return val_leapyear;
    }











    //test cases.
    #[cfg(test)]
mod tests{  
    use super :: *;
    #[test]
    fn check_leapyear()
        {
        assert_eq!(find_leapyear(2400),true);
        assert_eq!(find_leapyear(1800),false);
        assert_eq!(find_leapyear(2020),true);
        assert_eq!(find_leapyear(1900),false);
        assert_eq!(find_leapyear(1800),false);
        assert_eq!(find_leapyear(2500),false);
        }
        }