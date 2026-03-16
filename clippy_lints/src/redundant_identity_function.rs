use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::is_body_identity_function;
use rustc_abi::ExternAbi;
use rustc_hir::intravisit::FnKind;
use rustc_hir::{Body, FnDecl, FnRetTy, Impl, ItemKind, Node, TyKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::Span;
use rustc_span::def_id::LocalDefId;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for function definitions whose body simply returns
    /// their single parameter unchanged, i.e., identity functions.
    ///
    /// ### Why is this bad?
    /// The standard library already provides `std::convert::identity`
    /// for this purpose. A custom identity function is redundant.
    ///
    /// ### Limitations
    /// This lint only fires on functions with no trait bounds, no `impl Trait`
    /// parameters or return types, and no special ABI or safety modifiers.
    /// It does not fire on trait implementations or default trait methods,
    /// since those are required by the trait contract.
    ///
    /// Note that removing a named function requires updating all call sites,
    /// and may change function pointer types (a concrete `fn(u32) -> u32` is a
    /// distinct type from `std::convert::identity::<u32>`).
    ///
    /// ### Example
    /// ```no_run
    /// fn my_id<T>(x: T) -> T {
    ///     x
    /// }
    ///
    /// let result = my_id(42);
    /// ```
    ///
    /// Use instead:
    /// ```no_run
    /// // Remove the custom function and use `std::convert::identity` at call sites:
    /// let result = std::convert::identity(42);
    /// ```
    #[clippy::version = "1.96.0"]
    pub REDUNDANT_IDENTITY_FUNCTION,
    restriction,
    "defining a function that is equivalent to `std::convert::identity`"
}

declare_lint_pass!(RedundantIdentityFunction => [REDUNDANT_IDENTITY_FUNCTION]);

impl<'tcx> LateLintPass<'tcx> for RedundantIdentityFunction {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        fn_kind: FnKind<'tcx>,
        fn_decl: &FnDecl<'tcx>,
        body: &Body<'tcx>,
        span: Span,
        def_id: LocalDefId,
    ) {
        // Check item functions and methods, but not closures.
        let header = match fn_kind {
            FnKind::ItemFn(.., header) => header,
            FnKind::Method(_, sig) => sig.header,
            FnKind::Closure => return,
        };

        // Skip unsafe functions.
        if header.is_unsafe() {
            return;
        }

        // Skip async functions — they return a Future, not the value directly.
        if header.is_async() {
            return;
        }

        // Skip non-Rust ABI functions (extern "C", etc.).
        if header.abi != ExternAbi::Rust {
            return;
        }

        // Skip if generated from a macro expansion.
        if span.from_expansion() {
            return;
        }

        // Skip trait implementations and default trait methods — required by contract.
        let hir_id = cx.tcx.local_def_id_to_hir_id(def_id);
        if let Node::Item(item) = cx.tcx.parent_hir_node(hir_id)
            && matches!(
                item.kind,
                ItemKind::Impl(Impl { of_trait: Some(_), .. }) | ItemKind::Trait(..)
            )
        {
            return;
        }

        // Skip functions with #[no_mangle] or #[export_name].
        if cx.tcx.codegen_fn_attrs(def_id).contains_extern_indicator() {
            return;
        }

        // Skip functions with more than one type parameter — `identity` has exactly one.
        let generics = cx.tcx.generics_of(def_id);
        if generics.own_params.iter().filter(|p| p.kind.is_ty_or_const()).count() > 1 {
            return;
        }

        // Skip functions with non-Sized trait bounds or where clauses — they constrain
        // callers in ways `std::convert::identity` does not.
        let dominated_by_sized = cx.tcx.lang_items().sized_trait();
        if cx
            .tcx
            .explicit_predicates_of(def_id)
            .predicates
            .iter()
            .any(|(pred, _)| {
                if let rustc_middle::ty::ClauseKind::Trait(tr) = pred.kind().skip_binder() {
                    // Sized bounds are fine — `identity` also has an implicit Sized bound
                    dominated_by_sized != Some(tr.def_id())
                } else {
                    // Any other predicate kind (lifetime bounds, projections, etc.)
                    true
                }
            })
        {
            return;
        }

        // Skip functions with `impl Trait` in argument or return position —
        // `std::convert::identity` cannot express opaque types.
        if fn_decl.inputs.iter().any(|ty| matches!(ty.kind, TyKind::OpaqueDef(..)))
            || matches!(fn_decl.output, FnRetTy::Return(ty) if matches!(ty.kind, TyKind::OpaqueDef(..)))
        {
            return;
        }

        if is_body_identity_function(cx, body) {
            span_lint_and_help(
                cx,
                REDUNDANT_IDENTITY_FUNCTION,
                span,
                "this function is equivalent to `std::convert::identity`",
                None,
                "consider using `std::convert::identity` instead",
            );
        }
    }
}
