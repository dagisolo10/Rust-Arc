struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

fn main() {
    let my_box = ShippingBox {
        depth: 1,
        width: 2,
        height: 3,
    };

    let box_depth = my_box.depth; // 1
    let box_width = my_box.width; // 2
    let box_height = my_box.height; // 3

    println!("the box depth is {}", box_depth);
    println!("the box width is {}", box_width);
    println!("the box height is {}", box_height);
}
