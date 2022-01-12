pub mod flexbox;

use crate::flexbox::FlexItem;
use crate::flexbox::FlexBox;
use crate::flexbox::Direction;

fn main() {
    let mut root = FlexItem::new(100, 240);
    root.direction = Direction::Column;
    let mut child1 = FlexItem::new(60, 30);
    let mut child2 = FlexItem::new(60, 0);
    let mut child3 = FlexItem::new(60, 0);

    child1.grow = 0;
    child2.grow = 1;
    child3.grow = 2;


    root.add(child1);
    root.add(child2);
    root.add(child3);

    let mut layout = FlexBox::new();
    layout.layout(&mut root);

    print!("{:#?}", root);

}


