use bumpalo::Bump;
use ouroboros::self_referencing;

use id_arena::{Arena as IdArena, Id};

pub struct IdContainer {
    pub memory: IdArena<App>,
    pub app: Id<App>,
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

    #[borrows(mut memory)]
    pub app: &'this mut App,
}

impl BumpContainer {
    pub fn my_new() -> Self {
        BumpContainerBuilder {
            memory: Bump::new(),
            app_builder: |memory: &mut Bump| {
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
    fn test_mut_for_both() {
        // id_arena
        let mut id_container = IdContainer::new();
        let id_app = &mut id_container.memory[id_container.app];
        assert_eq!(id_app.name, "hello");
        id_app.name = "world".to_string();

        // bumpalo + ouroboros
        let mut bump_container = BumpContainer::my_new();
        let bump_app = bump_container.with_app(|app| app);
        assert_eq!(bump_app.name, "hello");

        bump_container.with_mut(|fields| {
            (**fields.app).name = "world".to_string();
        });

        let bump_app = bump_container.with_app(|app| app);

        assert_eq!(id_app.name, "world");
        assert_eq!(bump_app.name, "world");

        bump_container.with_app_mut(|app| {
            (**app).name = "world2".to_string();
        });

        let bump_app = bump_container.with_app(|app| app);
        assert_eq!(bump_app.name, "world2");
    }
}
