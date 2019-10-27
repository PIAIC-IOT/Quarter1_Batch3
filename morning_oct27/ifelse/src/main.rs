fn main() { //main start
    let check :u8 = 10;
    if check < 10 {  
        println!("We are in if block");

    } else {  //if end  else start
        println!("We are in ELse block");
    } //else end

    let age =20;
    let gender = "Male";

    if age<35 && gender=="Male"{
        println!("Your age is < 35 and you are Male");
    } else {
        println!("Your age is > 35 OR you are not Male");
    }

    if age > 35 || gender =="Female"{
        println!("Your age is > 35 Or you are Female");
    } else {
        println!("Your age is < 35 and you are Male");
    }


    println!("End of Program");


    let data:u8 = 33;
    if data+7==40{
    // 33+7==40
    // 40==40
        println!("Daud from Dera Ghazi Khan");
    } else if data+5==40{
        println!("Hasaan from Fasialabad");
    } else {
        println!("Tayyab from Narowal");
    }


    if data%5 == 0 {
     //3==0
     //false        
        println!("{} is divisible by 5",data);
    } else {
        println!("{} is not divisible by 5",data);
    }

    let number =3;
    if number ==1 {
        println!("One");
    } else if number ==2 {
        println!("Two");
    } else if number ==3 {
        println!("Three");
    } else {
        println!("Not Found");
    }

} //main end
