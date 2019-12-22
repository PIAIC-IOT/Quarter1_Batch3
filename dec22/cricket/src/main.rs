#[derive(Debug)]
struct Team {
    country:String,
    score:u16,
}
impl Team {
    fn high(self,other:Team)-> Team {
        if self.score > other.score {
        // team1.score > team2.score
            self //team1.score
        }
        else
        {
            other //team2.score
        }

    } 

    fn print(&self){
        println!("{}",self.score);
    }  
}



fn main() {
    let team1 = Team {
        country:"Pakistan".to_string(),
        score:435,
    };
    let team2 = Team {
        country:"Sri Lanka".to_string(),
        score:271,
    };
    team1.print();
    team2.print();

  //let today_high = team1.high();
    let today_high = team1.high(team2);
    
    println!("in main : {:#?}",today_high);
}
