use pealn::{pealn};

fn main() {

    let name  = "Subham Shaw";
    let age = 25;

    pealn!("[yellow,bold](Name) : [bold,underline]({}) " , name );
    
    pealn!("[yellow,bold](Age) : [bold]({}) " , age );

}

