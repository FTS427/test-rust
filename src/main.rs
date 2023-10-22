fn main() {
    calc();
}

fn calc(){
    print!("Hello Rust!");
    
    fn plus(a:i64, b:i64){
        let c:i64=a+b;
        print!("{} + {} = {}",a,b,c);
    }

    fn subt(a:i64,b:i64){
        let c:i64=a-b;
        print!("{} - {} = {}",a,b,c);
    }

    fn mutl(a:i64,b:i64) {
        let c:i64=a*b;
        print!("{} * {} = {}",a,b,c);
    }
    
    fn divi(a:i64,b:i64) {
        let c:i64=a/b;
        if a==0 {
            error();
        }
        else {
            print!("{} / {} = {}",a,b,c);
        }
    }
    
}

fn error() {
    print!("\033[31;1mERROR!");
}