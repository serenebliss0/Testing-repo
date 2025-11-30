
fn main()

{
   /*  'comment_on_a_loop_like_this: loop{
        println!("Konnichiwa!");
    }
*/
    let lag_array1 = [10,20,30,40,50];

    let mut index = 0;

    /*
    while index < lag_array1.len()
    {
        println!("The value at {index} is {}", lag_array1[index]);
        index+=1;
    }

    index = 0;
     */
    //--or--

    let mut lag_array2 = vec![10,20,30,40,50,60,70,80,90,100,110];

    for lagging_number in 0..lag_array2.len()
    {
        println!("The value is {}", lag_array2[lagging_number]);
    }

    for number in (1..4).rev()
    {
        println!("{number}");
    }

    lag_array2.pop();
    println!("{:?}", lag_array2);
}

