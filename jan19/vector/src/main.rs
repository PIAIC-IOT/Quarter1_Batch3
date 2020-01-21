fn main() {
    let mut data : Vec<i32> = Vec::new();
    data.push(22);
    data.push(45);
    data.push(25);
    data.push(33);
    data.push(44);
    println!("{:?}",data);

    for a in &mut data {
        * a += 5;
    }
    println!("{:?} ",data);
    
    data[1]=55;
    println!("{:?}",data);
    data.pop();
    println!("{:?}",data);
    data.clear();
    println!("{:?}",data);

    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}",v);
    let temp = v.get(1);
    println!("{:?}",temp);

}
