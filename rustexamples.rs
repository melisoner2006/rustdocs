fn main(){
  println!("hello, world!");
  let x = 5;
  println!("{}", x);
  let name = "melis";
  println!("{}", name);

  /* The following is not supported
  *println!("{}", name*x);
  */

  //try some patterns:
  println!("{subject}{verb}{object}",
    subject=" melis ",
    object=" rust ",
    verb=" is learning "
  );

  println!("{0} likes {1} but {1} doesn't know {0} likes {1}", "Melis", "David");

    println!("Every {} of {:b} turkish people will be punished by the future generation for damaging the turkish values.",5, 2);

    //create a structure which contains an i32, and name it Structure
    struct Structure(i32);

    //Have to learn Traits
    //let myStructure = Structure(5);
    //println!("This struct {} won't print...", myStructure);
}
