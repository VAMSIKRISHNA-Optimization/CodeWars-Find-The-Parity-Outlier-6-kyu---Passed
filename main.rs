fn find_outlier(values: &[i32]) -> i32 
{
    let evens: Vec<i32> = values.clone().iter().filter(|&&x| x.abs()%2 == 0).cloned().collect();
    let odds : Vec<i32> = values.clone().iter().filter(|&&x| x.abs()%2 == 1).cloned().collect();
    
    if evens.len() > odds.len()
    {
        odds[0]
    }
    else
    {
        evens[0]
    }
    
}

fn main ()
{
    println!("{:?}", find_outlier(&[std::i32::MAX, 0, 1] ));
}
