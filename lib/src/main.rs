use simplecs::{Component, ComponentStorageBuilder, With, Without};

#[derive(Component)]
pub struct PlayerTag;
#[derive(Component, Debug)]
pub struct HP { current: f32, max: f32 }

macro_rules! test {
    ($num:expr) => {
        let mut vec: Vec<String> = Vec::with_capacity($num);
        let mut str = String::with_capacity(16);
        for i in 0..$num {
            str.clear();
            str.push('_');
            str.push(i);
            vec.push(str);
        }
        test!(vec.join(","));
    };
    
    ($first:ident) => {
        println!(stringify!($first));
    };

    ($first:ident $(,$rest:ident),*) => {
        println!(stringify!($first $(,$rest),*));
        test!($($rest),*)
    };
}

fn main() {
    test!(2);
        
    /*let mut cs = ComponentStorageBuilder::<u8>::new()
        .with::<PlayerTag>()
        .with::<HP>()
        .build();
    cs.add_component(1, PlayerTag);
    cs.add_component(1, HP { current: 10.0, max: 10.0 });
    cs.add_component(2, PlayerTag);
    dbg!(cs.query::<(With<PlayerTag>, Without<HP>)>());*/
}