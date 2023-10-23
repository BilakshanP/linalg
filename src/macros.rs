#[macro_export]
macro_rules! insert_functions {
    ( $m:ident, $( $function:ident ),* ) => {
        $( $m.add_function(wrap_pyfunction!( $function, $m )?)?; )*
    };
}

#[macro_export]
macro_rules! insert_submodules {
    ( $m:ident, $( $module:ident ),* ) => {
        $( $m.add_wrapped(wrap_pymodule!( $module ))? )*
    };
}

#[macro_export]
macro_rules! insert_classes {
    ( $m:ident, $( $class:ty ),* ) => {
        $( $m.add_class::< $class >()?; )*
    };
}