use rand::Rng;
#[derive(Debug)]
pub struct Value {
    pub _ball: i32,
    pub _position: i32,
}

fn main() {
    let mut v = vec![];

    for n in 1..30 {
        v.push( Value {_ball: n, _position: 0} );
    }

    let mut v1_iter = v.iter();
  


    fn set_all_values(a: i32, b: i32) {
        // hacer un vector y modificar su valor en el iter
            let mut all_balls:Vec<Value> = {_ball: a, _position: b};
            
            for val in 1..16 { 
                let mut rng = rand::thread_rng();
                let random_bool = rng.gen::<bool>();

                // if random_bool == true { all_balls.push(Value {_ball: a, _position: b + 1})} else { all_balls.push(Value {_ball: a, _position: b + 1}) };
            }


    }


    for val in v1_iter {
        set_all_values(val._ball, val._position);
    }

    println!("{:?}", v1_iter);
}
