/// Defines unit structs that can be used as actions.
/// To use more complex data types as actions, use `impl_actions!`
#[macro_export]
macro_rules! actions {
    ($namespace:path, [ $($name:ident),* $(,)? ]) => {
        $(
            /// The `$name` action see [`gpui::actions!`]
            #[derive(::std::cmp::PartialEq, ::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug, gpui::private::serde_derive::Deserialize)]
            #[serde(crate = "gpui::private::serde")]
            pub struct $name;

            gpui::__impl_action!($namespace, $name,
                fn build(_: gpui::private::serde_json::Value) -> gpui::Result<::std::boxed::Box<dyn gpui::Action>> {
                    Ok(Box::new(Self))
                }
            );

            gpui::register_action!($name);
        )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_action {
    ($namespace:path, $name:ident, $build:item) => {
        impl gpui::Action for $name {
            fn name(&self) -> &'static str
            {
                concat!(
                    stringify!($namespace),
                    "::",
                    stringify!($name),
                )
            }

            fn debug_name() -> &'static str
            where
                Self: ::std::marker::Sized
            {
                concat!(
                    stringify!($namespace),
                    "::",
                    stringify!($name),
                )
            }

            $build

            fn partial_eq(&self, action: &dyn gpui::Action) -> bool {
                action
                    .as_any()
                    .downcast_ref::<Self>()
                    .map_or(false, |a| self == a)
            }

            fn boxed_clone(&self) ->  std::boxed::Box<dyn gpui::Action> {
                ::std::boxed::Box::new(self.clone())
            }

            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }
        }
    };
}

mod no_action {
    use crate as gpui;

    actions!(zed, [NoAction]);
}
