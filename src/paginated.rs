pub struct GridConfig {
    
}

type RenderItem<T> = fn(&T) -> ();

pub fn paginated<T>(items: Vec<T>, render_item: RenderItem<T>) {
    for item in items {
        render_item(&item);
    }
}