fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = ["item"; 100];
    // let a = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam eget mauris placerat, malesuada ante at, aliquam lacus. Ut eleifend posuere augue ut cursus. Phasellus metus nisl, sollicitudin vitae fermentum eu, interdum sit amet felis. Maecenas non gravida elit. Aenean sit amet odio id neque finibus ultricies sed sit amet nisi. Aliquam vestibulum imperdiet massa, in gravida augue accumsan vitae. Praesent eu lectus euismod, dictum turpis id, accumsan dui.";

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
