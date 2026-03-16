use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::is_body_identity_function;
use rustc_abi::ExternAbi;
use rustc_hir::intravisit::FnKind;
use rustc_hir::{Body, FnDecl, Impl, ItemKind, Node};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::Span;
use rustc_span::def_id::LocalDefId;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for function definitions that are equivalent to `std::convert::identity`.
    ///
    /// ### Why is this bad?
    /// `std::convert::identity` already provides this functionality. Defining a custom
    /// identity function adds unnecessary code and may confuse readers.
    ///
    /// ### Example
    /// ```no_run
    /// fn my_id<T>(x: T) -> T {
    ///     x
    /// }
    /// ```
    ///
    /// Use instead:
    /// ```no_run
    /// // Use `std::convert::identity` directly at call sites.
    /// let _ = std::convert::identity(42);
    /// ```
    #[clippy::version = "1.96.0"]
    pub REDUNDANT_IDENTITY_FUNCTION,
    complexity,
    "defining a function that is equivalent to `std::convert::identity`"
}

declare_lint_pass!(RedundantIdentityFunction => [REDUNDANT_IDENTITY_FUNCTION]);

impl<'tcx> LateLintPass<'tcx> for RedundantIdentityFunction {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        fn_kind: FnKind<'tcx>,
        _fn_decl: &FnDecl<'tcx>,
        body: &Body<'tcx>,
        span: Span,
        def_id: LocalDefId,
    ) {
        // Only check item functions and methods, not closures.
        let header = match fn_kind {
            FnKind::ItemFn(.., header) => header,
            FnKind::Method(..) | FnKind::Closure => return,
        };

        // Skip unsafe functions.
        if header.is_unsafe() {
            return;
        }

        // Skip non-Rust ABI functions (extern "C", etc.).
        if header.abi != ExternAbi::Rust {
            return;
        }

        // Skip trait implementations — the user is required to provide the function.
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

        // Skip if generated from a macro expansion.
        if span.from_expansion() {
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
