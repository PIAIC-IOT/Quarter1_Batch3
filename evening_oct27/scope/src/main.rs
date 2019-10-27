fn main() {  //start main
    println!("Welcome in main scope");
    let score = [66,44,55];
    let team = ["Pak","SL","Aus"];
    println!("Team : {}, Score: {}",team[0],score[0]);
    println!("Team : {}, Score: {}",team[1],score[1]);
    println!("Team : {}, Score: {}",team[2],score[2]);
    {  //child 1
        println!("Welcome in Child 1");
        println!("Team : {}, Score: {}",team[2],score[2]);
        let child1=5;
        println!(" Child 1 : {}",child1);
        { //grand child
            println!("Grand Child 1 : {}",child1);
            println!("Grand Team : {}, Score: {}",team[2],score[2]);

        } //grand child
    } //child end
    

    println!("End of Main scope");
    println!("Team : {}, Score: {}",team[2],score[2]);
    
} //end main
