#[macro_export]
macro_rules! insert_functions {
    ( $m:ident, $( $funcs:ident ),* ) => {
        $( $m.add_function(wrap_pyfunction!( $funcs, $m )?)?; )*
    };
}

#[macro_export]
macro_rules! insert_submodule {
    ( $m:ident, $( $mods:ident ),* ) => {
        $( $m.add_wrapped(wrap_pymodule!( $mods ))? )*
    };
}