fn main() {
    //루프 실습
    let mut counter=0;//그냥 타입만 정해주고 값 할당 안해주면 컴파일 에러가 발생한다.
    let result=loop{
        counter+=1;
        if counter==10{
            break counter*2;
        }
    };
    println!("The result is {result}");

    //루프 라벨: 바깥쪽 루프 대신 라벨이 적힌 특정한 루프에 적용되도록 할 수 있다.
    //루프 라벨은 반드시 작은 따옴표 ' 로 시작해야 한다.

    let mut count=0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining=10;
        loop {
            println!("remaining = {remaining}");
            if remaining==9{
                break;
            }
            if count==2{
                break 'counting_up;//바깥 루프문도 탈출
            }
            remaining-=1;
        }
        count+=1;
    }
    println!("End count= {count}");

    //while 3-3 카운트 다운 예제
    let mut number=3;
    while number!=0{
        println!("{number}");

        number-=1;
    }
    println!("LIFTOFF!!");

    let a=[10,20,30,40,50];
    let mut index=0;

    while index<5 { //매번 a 배열에 변경이 가해질 때 마다 조건문 수정해줘야 해서 번거롭다 -> 패닉 우려있음
        println!("the value is {}", a[index]);

        index+=1;
    }

    //더 나은 해결책
    for element in a {
        println!("the value is: {element}");
    }

    //while 3-3도 안정성을 위해 다음과 같이 바꿀 수 있다.
    for number in (1..4).rev(){//1~3까지 반복(4이전까지), .rev()는 역순으로
        println!("{number}!");
    }
    println!("LITFOFF!!");
}
