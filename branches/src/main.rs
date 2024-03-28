fn main() {
    let number=7;
    if number < 5 {
        println!("condition was true");
    }
    else{
        println!("condition was false");
    }

    //if number{
        //println!("number was seven"); boolean type이 아니기 때문에 에러발생
    //}
    if number!=0{
        println!("number was something other than zero");
    }
    //코드 블록은 블록 안의 마지막 표현식을 계산하고, 숫자는 그 자체로 표현식이다!
    //if 표현식의 각 갈래의 결과값은 같은 타입이어야 한다.

    let condition=true;
    let number=if condition {5} else {6};

    //컴파일 에러 발생 -> 에러가 발생하는 이유는 변수가 가질 수 있는 타입이 오직 하나이기 때문
    //let number= if condition {5} else {"six"}; //러스트는 런타임에서 변수 타입을 정의할 수 없다 
    //-> 컴파일러가 더 복잡해지고 보장할 수 있는 것이 줄어들기 때문
    
    println!("The value of number is : {number}");

}
