#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width*self.height
    }
    
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width>other.width&&self.height>other.height
    }
}

fn main() {
    //let width1=30;
    //let height1=50;

    let rect1=Rectangle{
        width:30,
        height:50,
    };
    let rect2=Rectangle{
        width:10,
        height:40,
    };
    let rect3=Rectangle{
        width:60,
        height:45,
    };

    println!(
        "The area of the rectagle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}


fn area(rect:&Rectangle)-> u32 {//구조체의 대여
    rect.width*rect.height
}
