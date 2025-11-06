use std::io;
use std::f64::consts::E;

fn main(){
    println!("Choose a function num:\n 1\t plus\n 2\t minus\n 3\t multiplication\n 4\t division\n 5\t friction\n 6\t integrate_trapezoid\n 7\t derivative\n");
    let mut function_num=String::new();
    io::stdin().read_line(&mut function_num).unwrap();
    let function_num=function_num.trim().parse::<i32>().unwrap();


    if function_num==6{
         // === 动态函数积分部分 ===
        println!("Please input the expression of f(x), e.g. x*x, x.sin(), x.exp(), 1/x");
        let mut expr=String::new();
        io::stdin().read_line(&mut expr).unwrap();
        let expr=expr.trim().to_string();

        println!("Please input lower limit a1:");
        let mut a1=String::new();
        io::stdin().read_line(&mut a1).unwrap();
        let a1= a1.trim().parse::<f64>().unwrap();

        println!("Please input upper limit a2:");
        let mut a2 = String::new();
        io::stdin().read_line(&mut a2).unwrap();
        let a2 = a2.trim().parse::<f64>().unwrap();


        let result=integrate_trapezoid_dynamic(&expr, a1, a2, 1000);
        println!("∫_{}^{} {} dx ≈ {}", a1, a2, expr, result);
        return;

    }


    if function_num==7{
        // === 动态函数derivative部分 ===
        println!("Please input the expression of f(x), e.g. x*x, x.sin(), x.exp(), 1/x");
        let mut expr=String::new();
        io::stdin().read_line(&mut expr).unwrap();
        let expr=expr.trim().to_string();

        println!("Please input lower limit a1:");
        let mut a1=String::new();
        io::stdin().read_line(&mut a1).unwrap();
        let a1= a1.trim().parse::<f64>().unwrap();

        // println!("Please input upper limit a2:------>useless, just for uniform because i am very cai");
        // let mut a2 = String::new();
        // io::stdin().read_line(&mut a2).unwrap();
        // let a2 = a2.trim().parse::<f64>().unwrap();

        let result=derivative_dynamic(&expr,a1,0.001);
        println!("f(x)={}, x={}, f'(x)={}",expr,a1,result);
        return;

    }


    // === 普通计算部分 ===
    println!("Please insert num a1");
    let mut a1 = String::new();
    io::stdin().read_line(&mut a1).unwrap();
    let a1 = a1.trim().parse::<f64>().unwrap();

    println!("Please insert num a2");
    let mut a2 = String::new();
    io::stdin().read_line(&mut a2).unwrap();
    let a2 = a2.trim().parse::<f64>().unwrap();

    match function_num {
        1 => println!("plus:{}", plus_al(a1, a2)),
        2 => println!("minus:{}", minus_al(a1, a2)),
        3 => println!("multiplication:{}", multiplication_al(a1, a2)),
        4 => println!("division:{}", division_al(a1, a2)),
        5 => println!("friction:{}", friction_al(a1, a2)),
        _ => println!("Error in function_num: invalid."),
    }

}

// ========== 基本运算 ==========
fn plus_al(a: f64, b: f64) -> f64 { a + b }
fn minus_al(a: f64, b: f64) -> f64 { a - b }
fn multiplication_al(a: f64, b: f64) -> f64 { a * b }
fn division_al(a: f64, b: f64) -> f64 { a / b }
fn friction_al(x: f64, n: f64) -> f64 {
    let mut y = 1.0;
    let n_int = n as i32;
    for _ in 0..n_int {
        y *= x;
    }
    y
}

// ========== 动态积分部分 ==========
fn integrate_trapezoid_dynamic(expr:&str, a:f64, b:f64, n:i32)->f64{
    let f=select_function(expr);
    integrate_trapezoid(f,a,b,n)

}

fn integrate_trapezoid<F>(f:F, a:f64, b:f64, n:i32)->f64
where F:Fn(f64)->f64,
{
    let n_f64=n as f64;
    let h=(a-b)/n_f64;
    let mut total = 0.5*(f(a)+f(b));
    for i in 1..n{
        let i_f64=i as f64;
        total+=f(a+i_f64*h);
    }
    total*h
}

// ========== 动态函数选择 ==========
fn select_function(expr:&str)->Box<dyn Fn(f64)->f64>{
    match expr{
        "x*x"=>Box::new(|x| x*x),
        "x"=>Box::new(|x| x),
        "1/x"=>Box::new(|x| 1.0/x),
        "x.sin()"=>Box::new(|x| x.sin()),
        "x.cos()"=>Box::new(|x| x.cos()),
        "x.exp()"=>Box::new(|x| x.exp()),
        "e^x"=>Box::new(|x| E.powf(x)),
        _=>{
            println!("Unsupported function '{}', using x*x as default.", expr);
            Box::new(|x| x * x)
        }
    }
}


fn derivative_dynamic(expr:&str,a:f64,h:f64)->f64{
    let f=select_function(expr);
    derivative(f,a,h)
}

fn derivative<F>(f:F,a:f64,h:f64)->f64
where F:Fn(f64)->f64,
{
    (f(a+h)-f(a-h))/(2.0*h)
}