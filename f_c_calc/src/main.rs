use std::io;

fn main() {
    let mut num=String::new();
    loop{
        println!("Enter the number(1:Fahrenheit to Celsius, 2:Celsius to Fahrenheit)");

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        //let num:u32=num.trim().parse().expect("Please type a number!");
        //잘못된 값 처리하기
        let num:u32= match num.trim().parse() {
            Ok(num)=>num,
            Err(_)=> continue,
        };

        let result = f_c_calc(num);

        println!("{result}");
        break;
    }
}

fn f_c_calc(num:u32) -> f32{
    if num==1 {
        println!("Enter your Fahrenheit");
        let mut f=String::new();
        //입력받은 f값 실수로 변환하기
        io::stdin()
        .read_line(&mut f)
        .expect("Failed to read line");

        let f:f32=f.trim().parse().expect("Please type a number!");

        //값 계산하기
        let c=(f-32.0)*0.55;

        c

    } else if num==2 {
        //입력받은 f값 실수로 변환하기
        println!("Enter your Celsius");

        let mut c=String::new();
        io::stdin()
        .read_line(&mut c)
        .expect("Failed to read line");

        let c:f32=c.trim().parse().expect("Please type a number!");
        
        let f=c*1.8+32.0;

        f
    } else{
        panic!("Invalid Choice!")
    }
}
