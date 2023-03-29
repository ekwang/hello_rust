use std::io;

fn transform(temper: f32, standard: &str) -> Option<f32>{
    if standard == "C" {
        // Transform to F from C
        Some((temper * 1.8) + 32f32)
    } else if standard == "F" {
        // Transform to C from F
        Some((temper - 32f32) * 5f32/9f32)
    } else {
        println!("{standard} is unsupported format");
        None
    }
}

fn main() {
    let mut input = String::new();
    let number: f32;
    let mut standard: &str;
    let trans_num;

    loop {
        println!("Please input temperature to transform: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read temperature!");

        let len = input.len();

        // println!("input:{input}, len:{len}, num:{ }, standard:{ }", &input[0..len-2], &input[len-2..len-1]); 
        // dbg!("{ }", &input);

        standard = &input[len-2..len-1];
        number = match input[0..len-2].parse() {
            Ok(num) => num,
            Err(_) => {println!("wrong value"); continue},
        };
        break;
    };

    trans_num = transform(number, standard);

    match trans_num {
        Some(num) => {
            println!("transform result: {num}{ }", if standard == "F" {"C"} else {"F"} );
        },
        None => {
            println!("Fail to transform");
        }
    }
}
