fn main() {
    let lyrics=[
        "A partridge in a peer tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    let count=[
        "first", "second", "thrid", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh", "twelfth"
    ];

    println!("<The Twelve Days if Christmas>");
    println!();
    
    for i in 0..12{
        println!("On the {} day of Christmas,", count[i]);
        println!("my true love gave to me");
        for j in (0..i+1).rev(){//.rev()이런거 붙으면 ()가 필요하고 없으면 괄호 없이 사용 가능한 것 같다.
            if j != 0 {
                println!("{},",lyrics[j])
            }
            else{
                println!("And {}.",lyrics[j])
            }
        }
        println!();
    }

}
