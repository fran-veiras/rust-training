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
  


    fn set_all_values(a: i32, b: i32)  -> Value {
            let mut balls = Value {_ball: a, _position: b};
            
            for val in 1..16 { 
                let mut rng = rand::thread_rng();
                let random_bool = rng.gen::<bool>();

                if random_bool == true { balls._position = balls._position + 1 } else { balls._position = balls._position - 1 };
            }

            return balls
    }

    let mut response: Vec<Value> = Vec::new();

    for val in v1_iter {
        let balls_with_position = set_all_values(val._ball, val._position);

        response.push(balls_with_position)
    }

    println!("{:?}", response);
}
