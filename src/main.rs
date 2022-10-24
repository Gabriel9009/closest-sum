fn main() {
   let arr: [i32; 4] = [-1, 2, 1, -4];
   let mut sum = 0;
   let target = 1;
   for i in 0..arr.len(){
    sum = sum + arr[i];
    if sum ==target + 1{
        println!("{}", i);
    }else if sum == 0{
        println!("{}", sum);
        break;
    }
    }
    }

