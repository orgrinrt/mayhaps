pub use paste::paste as __mayhaps_paste;

#[macro_export]
macro_rules! maybe {
    ($name:ident from ($opt_var:expr) exists $body:block) => {
        $crate::__mayhaps_paste! {
            let [< maybe_ $name >] = $opt_var;
            if let Some($name) = [< maybe_ $name >] $body
        }
    };
    ($name:ident from ($opt_var:expr) exists { $($body:tt)* } else $else_body:block) => {
        $crate::__mayhaps_paste! {
            let [< maybe_ $name >] = $opt_var;
            if let Some($name) = [< maybe_ $name >] $body $else_body
        }
    };
}
