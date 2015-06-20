

fn combine1(v1: &Vec<char>, v2: &Vec<char>) -> Vec<char> {
    let mut v3 = Vec::new();
    for i in 0..v1.len() {
        v3.push(v1[i]);
        v3.push(v2[i]);
    }
    v3
}

fn combine2(v1: &Vec<char>, v2: &Vec<char>) -> Vec<char> {
    let mut v3 = Vec::new();
    for (i,j) in v1.iter().zip(v2) {
        v3.push(*i);
        v3.push(*j);
    }
    v3
}

fn main() {
    let v1 = vec!('a','b','c');
    let v2 = vec!('1','2','3');
    println!("{:?}",combine1(&v1, &v2));
    println!("{:?}",combine2(&v1, &v2));
}

/*
{ let y = &x ; println!("{}",y); }
println!("{}",x);
23:47 < klieth> when you used the brackets, you opened up a new scope
23:47 < klieth> inside that scope, you _borrowed_ the value of x, printed it
23:47 < klieth> and then it went out of scope, so x in un-borrowed and usable again
23:48 < klieth> is un-borrowed*
 */

