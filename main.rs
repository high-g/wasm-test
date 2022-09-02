const PI:f32 = 3.14;

fn main() {
  let arg = 1;
  println!("{}", arg);

  let mut arg2 = 2;
  println!("{}", arg2);

  arg2 = 3;
  println!("{}", arg2);

  println!("{}", PI);

  let numbers: [i32; 4] = [1,2,3,4];
  println!("{:?}", numbers);
  println!("{}", numbers[0]);

  let calc_num = add(1, 2);
  println!("{}", calc_num);


  let calc_num2 = twice(1, 2);
  let (res_1, res_2) = twice(1, 2);
  println!("{}, {}", calc_num2.0, calc_num2.1);
  println!("{}, {}", res_1, res_2);

  let blank = blank_func();
  println!("blankの値: {:?}", blank);


  // if文
  let num_if = 10;
  if num_if < 10 {
    println!("num_ifは10より小さい");
  } else if num_if == 10 {
    println!("num_ifは10");
  } else {
    println!("num_ifは10より大きい");
  }

  // loop文
  let mut num_loop = 1;
  loop {
    println!("num_loop: {}", num_loop);
    num_loop += 1;

    if num_loop == 10 {
      break;
    }
  }

  // while文
  let mut x = 0;
  while x < 10 {
    println!("x: {}", x);
    x += 1;
  }

  // for文
  for i in 0..5 {
    println!("{}", i);
  }
  for i in 0..=5 {
    println!("{}", i);
  }

  // for文で配列を回す
  let array = [1,2,3,4,5];
  for i in array.iter() {
    println!("{}", i);
  }

  // match文
  let num_match = 1;
  match num_match {
    1 => {
      println!("1です");
    }
    2 | 3 => {
      println!("2か3です");
    }
    _ => {
      println!("マッチしませんでした");
    }
  }

  // 三項演算子
  let v = if num_match > 3 { -1 } else {1};
  println!("{}", v);
}

fn add (a: i32, b: i32) -> i32 {
  return a + b
}

fn twice (a: i32, b: i32) -> (i32, i32) {
  return (a*2, b*2)
}

fn blank_func() -> () {
  return ()
}
