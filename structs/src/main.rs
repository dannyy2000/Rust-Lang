
// fn main() {
//     let rect = rectangle{
//         width : 30,
//         height : 20,
//     };

//     println!("rect is {rect:?}");
// }


// #[derive(Debug)]
// struct rectangle {
//     width: u32,
//     height:u32
// }



// 


#[derive(Debug)]
struct Rectangle {
    width :u32,
    height :u32
}

fn main(){
   let rect1 = Rectangle{
        width: 20,
        height: 30,

    };

    let rect2 = Rectangle {
        width: 10,
        height : 10,
    };


    let rect3 = Rectangle {
        width: 40,
        height : 50,
    };

    println!("can rectangle 1 hold rectangle 2 {}" ,{rect1.can_hold(&rect2)});
      println!("can rectangle 2 hold rectangle 3 {}" ,{rect2.can_hold(&rect3)});

   
}



impl Rectangle {
    fn can_hold(&self, others: &Rectangle) -> bool {
        self.width > others.width && self.height > others.height
    }
}



