window.SIDEBAR_ITEMS = {"constant":[["INIT_HOOK_PRIORITY","The priority of the startup hooks used to initialize bindings."]],"fn":[["bind","A shorthand for [`Bind`][]`::``define``().``init_with_bind``(...)`."],["bind_default","A shorthand for [`Bind`][]`::``define``().``init``(``default``).``finish``(cfg)`."],["bind_uninit","A shorthand for [`Bind`][]`::``define``().``init``(``MaybeUninit::uninit``).``finish``(cfg)`."],["fn_bind_map","Apply a function to an `impl `[`FnBind`][]`<_>`’s output."]],"struct":[["Bind","A defined binding."],["BindBorrow","A binder that gives `&T` to a bound function."],["BindBorrowMut","A binder that gives `&mut T` to a bound function."],["BindDefiner","The definer (static builder) for [`Bind`]."],["BindRef","A reference to a binding. Doubles as a binder."],["BindTable","Represents a permission to dereference [`BindRef`][]."],["BindTake","A binder that gives `T` to a bound function."],["BindTakeMut","A binder that gives `&'static mut T` to a bound function."],["BindTakeRef","A binder that gives `&'static T` to a bound function."],["FnBindMap","Applies a function to a [`FnBind`][]’s output."]],"trait":[["Binder","Represents a binder, which represents a specific way to access the contents of a binding from a runtime function."],["ExecutableDefiner","A trait for definer objects (static builders) for kernel objects that can spawn a thread that executes after the execution of all startup hooks is complete."],["ExecutableDefinerExt","An extension trait for [`ExecutableDefiner`]. Provides a method to attach an entry point with materialized bindings."],["FnBind","A trait for closures that can receive bindings materialized through specific binders (`Binder`)."],["UnzipBind","An extension trait for destructing [`Bind`][]`<_, (T0, T1, ...)>` into individual bindings `(Bind<_, T0>, Bind<_, T1>, ...)`."]]};