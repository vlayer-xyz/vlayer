/// Macro for generating structs representing filesystem data layouts.
/// It accepts three parameters:
///   * generated struct name,
///   * list of 'root directories' with initializer expressions,
///   * list of nested paths with assigned names.
///
/// Root directories are `PathBuf` fields of the generated struct.
/// There is a fallible `new()` implementation generated returning `Option<Self>`,
/// so the root directories' initializer expressions can use `?` operator.
/// Initializer expressions can reference other roots, but beware of the ordering.
/// Nested paths can be based on any root dir or another nested path.
///
/// Example:
/// ```
/// use guest_build_utils::data_layout;
/// data_layout!(MyLayout {
///     root: "/".into(),
/// } {
///     (root / "home") => home,
///     (home / "user") => user,
/// });
/// ```
/// Will generate the following code:
/// ```
/// struct MyLayout {
///     root: std::path::PathBuf,
/// }
///
/// impl MyLayout {
///     pub fn new() -> Option<Self> {
///         let root: std::path::PathBuf = "/".into();
///         Some(MyLayout {
///             root: root,
///         })
///     }
///
///     pub fn root(&self) -> &std::path::Path {
///         &self.root
///     }
///
///     pub fn home(&self) -> std::path::PathBuf {
///         self.root().join("home")
///     }
///
///     pub fn user(&self) -> std::path::PathBuf {
///         self.home().join("user")
///     }
/// }
/// ```
#[macro_export]
macro_rules! data_layout {
    ($struct_name:ident {
        $( $root_name:ident : $root_value:expr, )*
    } {
        $( ($parent_selector:ident / $path:expr) => $name:ident, )*
    }) => {
        // Struct definition
        pub struct $struct_name {
            $( pub $root_name: std::path::PathBuf ),*
        }

        impl $struct_name {
            // Constructor
            pub fn new() -> Option<Self> {
                $( let $root_name: std::path::PathBuf = $root_value; )*
                Some($struct_name {
                    $( $root_name, )*
                })
            }

            // Getter methods for each root dir
            $(pub fn $root_name(&self) -> &std::path::Path {
                &self.$root_name
            })*

            // Getter methods for each nested path
            $(pub fn $name(&self) -> std::path::PathBuf {
                self.$parent_selector().join($path)
            })*
        }
    };
}
