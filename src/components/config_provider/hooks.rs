use crate::components::config_provider::{
    context::{get_component_cls, ConfigContext},
    Direction, SpaceSize,
};
use leptos::prelude::*;
use web_sys::Element;

/// Hook to get component class name
pub fn use_component_cls(suffix: &str) -> String {
    get_component_cls(suffix)
}

/// Hook to get direction
pub fn use_direction() -> Option<ReadSignal<Direction>> {
    use_context::<ConfigContext>().map(|ctx| {
        let signal = RwSignal::new(ctx.get().direction);
        Effect::new(move |_| {
            signal.set(ctx.get().direction);
        });
        signal.read_only()
    })
}

/// Hook to get form config
pub fn use_form_config() -> Option<ReadSignal<bool>> {
    use_context::<ConfigContext>().map(|ctx| {
        let signal = RwSignal::new(ctx.get().form.require_mark);
        Effect::new(move |_| {
            signal.set(ctx.get().form.require_mark);
        });
        signal.read_only()
    })
}

/// Hook to get space config
pub fn use_space_config() -> Option<ReadSignal<f64>> {
    use_context::<ConfigContext>().map(|ctx| {
        let signal = RwSignal::new(match ctx.get().space.size {
            SpaceSize::Small => 8.0,
            SpaceSize::Middle => 16.0,
            SpaceSize::Large => 24.0,
            SpaceSize::Custom(size) => size,
        });
        Effect::new(move |_| {
            signal.set(match ctx.get().space.size {
                SpaceSize::Small => 8.0,
                SpaceSize::Middle => 16.0,
                SpaceSize::Large => 24.0,
                SpaceSize::Custom(size) => size,
            });
        });
        signal.read_only()
    })
}

/// Hook to get disabled state
pub fn use_disabled() -> Option<ReadSignal<bool>> {
    use_context::<ConfigContext>().map(|ctx| {
        let signal = RwSignal::new(ctx.get().component_disabled);
        Effect::new(move |_| {
            signal.set(ctx.get().component_disabled);
        });
        signal.read_only()
    })
}

/// Hook to get popup container
pub fn use_popup_container() -> Option<ReadSignal<Option<String>>> {
    use_context::<ConfigContext>().map(|ctx| {
        let signal = RwSignal::new(None);

        Effect::new(move |_| {
            if let Some(container) = ctx.get_untracked().get_popup_container() {
                let id = container.id();
                if !id.is_empty() {
                    signal.set(Some(id));
                } else {
                    // 如果元素没有 ID，生成一个唯一 ID 并设置
                    let unique_id = format!("popup-container-{}", js_sys::Math::random());
                    container.set_id(&unique_id);
                    signal.set(Some(unique_id));
                }
            }
        });

        signal.read_only()
    })
}

/// Helper function to get Element by ID
pub fn get_container_element(id: &str) -> Option<Element> {
    let window = web_sys::window()?;
    let document = window.document()?;
    document.get_element_by_id(id)
}
