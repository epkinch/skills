use std::env;

fn main(){
    let mut groups = [[""; 4]; 6];
    groups[0]=["Bob", "Carol", "Eric", "Matt"];
    groups[1]=["Jim", "Lucy", "Terry", "Brenda"];
    groups[2]=["Susan", "Brad", "Jim", "Matt"];
    groups[3]=["Sue", "Wendy", "Sam", "Brad"];
    groups[4]=["Kate", "Jack", "James", "Sydney"];
    groups[5]=["Mary", "John", "Ricky", "Wendy"];

    let member = env::args().nth(1).expect("expected command line argument");
    searchMember(&groups, &member);
}

fn searchMember(groups: &[[&str; 4]; 6], member: &str) {
    let mut flag: bool = false;
    for (i, group) in groups.iter().enumerate() {
       for (j, groupee) in group.iter().enumerate() {
            if member == *groupee {
                flag = true;
                print!("{} belongs to group {}", member, i+1);
                if j == 0 {
                    print!(" and is their group leader");
                }
                println!(".");
            }
       }
    }
    
    if !flag {
        println!("{} does not belong to any group.", member)
    }
}