use std::collections::HashMap;

fn main(){

    let mut capital = HashMap::new();

    capital.insert("Pakistan","Islamabad");
    //              index   ,  value  
    capital.insert("Punjab","Lahore");
    //              key   ,  value  
    capital.insert("Sindh","Karachi");

    capital.insert("Pakistan","New Islamabad");
    //              index   ,  value  
    capital.entry(("Pakistan")).or_insert("Old Islamabad");
    //              index   ,              value  
    capital.entry(("KP")).or_insert("Peshawar");
    //              index   ,         value  

    println!("{:#?}",capital);

    let pk = capital.get("Pakistan");
    println!("{:#?}",pk);

    let pn = capital.get("PUNJAB");
    println!("{:#?}",pn);

    let sn = capital["Sindh"];
    
    println!("{:#?}",sn);

    for (cp, capital) in &capital {
        println!("{}: {}", cp, capital);
    }


    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}",scores);
    println!("{:?}",scores.get(&String::from("Yellow")));
    println!("{:?}",scores[&String::from("Blue")]);




}