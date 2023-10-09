use bumpalo::Bump;
use ouroboros::self_referencing;

use id_arena::{Arena as IdArena, Id};

pub struct Container {
    memory: IdArena<App>,
    app: Id<App>,
}

impl Container {
    pub fn new() -> Self {
        let mut memory = IdArena::new();
        let app = memory.alloc(App {});
        Self { memory, app }
    }
}

#[self_referencing]
pub struct BumpContainer {
    pub memory: Bump,

    #[borrows(memory)]
    pub app: &'this App,
}

impl BumpContainer {
    fn my_new() -> Self {
        BumpContainerBuilder {
            memory: Bump::new(),
            app_builder: |memory: &Bump| {
                let app = memory.alloc(App {});
                app
            },
        }
        .build()
    }
}

#[derive(Debug)]
pub struct App {}

fn main() {
    let container = Container::new();
    let app = container.memory[container.app];
    let bump_container = BumpContainer::my_new();

    let app = bump_container.with_app(|app| app);
    println!("Hello, world!");
}
