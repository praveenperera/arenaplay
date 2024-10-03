use bumpalo::Bump;
use ouroboros::self_referencing;

use id_arena::{Arena as IdArena, Id};

pub struct IdContainer {
    memory: IdArena<App>,
    app: Id<App>,
}

impl IdContainer {
    pub fn new() -> Self {
        let mut memory = IdArena::new();
        let app = memory.alloc(App {
            name: "hello".to_string(),
        });
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
                let app = memory.alloc(App {
                    name: "hello".to_string(),
                });
                app
            },
        }
        .build()
    }
}

#[derive(Debug)]
pub struct App {
    pub name: String,
}

impl App {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn change_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        // id_arena
        let mut id_container = IdContainer::new();
        let id_app = &mut id_container.memory[id_container.app];
        id_app.name = "world".to_string();

        // bumpalo + ouroboros
        let mut bump_container = BumpContainer::my_new();
        let bump_app = &mut bump_container.with_app_mut(|&mut app: &mut &App| app);
        // let mut app = &mut bump_container.with_app_mut(|&mut app: &mut &App| app);
        // bump_app.change_name("world");

        assert_eq!(id_app.name, "world");
        assert_eq!(bump_app.name, "world");
    }
}
