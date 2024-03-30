use std::io;

fn main() {
    println!("Input n");
    let mut num=String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Please type a number!");
    let num:u32=num.trim().parse().expect("Please type a number!");
    let result=d_fib(num);

    println!("{result}");
}

fn d_fib(num:u32) -> u64{
    if num==0 {
        return 0;
    }
    if num==1 {
        return 1;
    }

    let mut pp:u64=0;
    let mut p:u64=1;
    let mut current:u64=0;//값을 꼭 아무 값으로라도 초기화 해줘야 컴파일이 된다.

    for _i in 2..num+1 { //_를 통해 사용되지 않는 변수가 있을 때 warning이 표시되는 것을 막을 수 있다.
        current=p+pp;
        pp=p;
        p=current;
    }
    current
}
