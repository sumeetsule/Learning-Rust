// Simple Function
/*
fn main(){
    //calling a function
    hello();
 }
 
 fn hello(){
    println!("Hi");
}*/



// Return Value
// Demonstrate the same with the return value (35000.00*5.0*6.4/100.00)
/*
fn main(){
    println!("Hi {}",calc_si());
 }
 
 fn calc_si()->f32 {
    35000.00*5.0*6.4/100.00
} */



// Call by Value
/*
fn main(){
    let no:i32 = 5;
    changeme(no);
    println!("Main Function:{}",no);
 }
 
 fn changeme(mut param_no: i32) {
    param_no = param_no * 0;
    println!("Inside the Function :{}",param_no);
}  */




// Call by Reference

fn main() {
    let mut no: i32 = 5;
    println!("Main fucntion initial value :{} -> {:p}", no,&no);
    changeme(&mut no);
    println!("Main function final value  is:{} -> {:p}", no,&no);
}

fn changeme(param_no: &mut i32) {
    println!("Changeme function initial value :{} -> {:p}", *param_no,&(*param_no));
    *param_no = 0; //de reference
    println!("Changeme function final value :{} -> {:p}", *param_no,&(*param_no));
} 
