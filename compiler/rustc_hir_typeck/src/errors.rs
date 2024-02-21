//! Errors emitted by `rustc_hir_typeck`.
use std::borrow::Cow;

use crate::fluent_generated as fluent;
use rustc_errors::{
    codes::*, AddToDiagnostic, Applicability, DiagnosticArgValue, DiagnosticBuilder,
    EmissionGuarantee, IntoDiagnosticArg, MultiSpan, SubdiagnosticMessageOp,
};
use rustc_macros::{Diagnostic, LintDiagnostic, Subdiagnostic};
use rustc_middle::ty::Ty;
use rustc_span::{
    edition::{Edition, LATEST_STABLE_EDITION},
    symbol::Ident,
    Span,
};

#[derive(Diagnostic)]
#[diag(hir_typeck_field_multiply_specified_in_initializer, code = E0062)]
pub struct FieldMultiplySpecifiedInInitializer {
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(hir_typeck_previous_use_label)]
    pub prev_span: Span,
    pub ident: Ident,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_return_stmt_outside_of_fn_body, code = E0572)]
pub struct ReturnStmtOutsideOfFnBody {
    #[primary_span]
    pub span: Span,
    #[label(hir_typeck_encl_body_label)]
    pub encl_body_span: Option<Span>,
    #[label(hir_typeck_encl_fn_label)]
    pub encl_fn_span: Option<Span>,
    pub statement_kind: ReturnLikeStatementKind,
}

pub enum ReturnLikeStatementKind {
    Return,
    Become,
}

impl IntoDiagnosticArg for ReturnLikeStatementKind {
    fn into_diagnostic_arg(self) -> DiagnosticArgValue {
        let kind = match self {
            Self::Return => "return",
            Self::Become => "become",
        }
        .into();

        DiagnosticArgValue::Str(kind)
    }
}

#[derive(Diagnostic)]
#[diag(hir_typeck_rustcall_incorrect_args)]
pub struct RustCallIncorrectArgs {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_yield_expr_outside_of_coroutine, code = E0627)]
pub struct YieldExprOutsideOfCoroutine {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_struct_expr_non_exhaustive, code = E0639)]
pub struct StructExprNonExhaustive {
    #[primary_span]
    pub span: Span,
    pub what: &'static str,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_method_call_on_unknown_raw_pointee, code = E0699)]
pub struct MethodCallOnUnknownRawPointee {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_functional_record_update_on_non_struct, code = E0436)]
pub struct FunctionalRecordUpdateOnNonStruct {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_address_of_temporary_taken, code = E0745)]
pub struct AddressOfTemporaryTaken {
    #[primary_span]
    #[label]
    pub span: Span,
}

#[derive(Subdiagnostic)]
pub enum AddReturnTypeSuggestion {
    #[suggestion(
        hir_typeck_add_return_type_add,
        code = " -> {found}",
        applicability = "machine-applicable"
    )]
    Add {
        #[primary_span]
        span: Span,
        found: String,
    },
    #[suggestion(
        hir_typeck_add_return_type_missing_here,
        code = " -> _",
        applicability = "has-placeholders"
    )]
    MissingHere {
        #[primary_span]
        span: Span,
    },
}

#[derive(Subdiagnostic)]
pub enum ExpectedReturnTypeLabel<'tcx> {
    #[label(hir_typeck_expected_default_return_type)]
    Unit {
        #[primary_span]
        span: Span,
    },
    #[label(hir_typeck_expected_return_type)]
    Other {
        #[primary_span]
        span: Span,
        expected: Ty<'tcx>,
    },
}

#[derive(Diagnostic)]
#[diag(hir_typeck_explicit_destructor, code = E0040)]
pub struct ExplicitDestructorCall {
    #[primary_span]
    #[label]
    pub span: Span,
    #[subdiagnostic]
    pub sugg: ExplicitDestructorCallSugg,
}

#[derive(Subdiagnostic)]
pub enum ExplicitDestructorCallSugg {
    #[suggestion(hir_typeck_suggestion, code = "drop", applicability = "maybe-incorrect")]
    Empty(#[primary_span] Span),
    #[multipart_suggestion(hir_typeck_suggestion, style = "short")]
    Snippet {
        #[suggestion_part(code = "drop(")]
        lo: Span,
        #[suggestion_part(code = ")")]
        hi: Span,
    },
}

#[derive(Diagnostic)]
#[diag(hir_typeck_missing_parentheses_in_range, code = E0689)]
pub struct MissingParenthesesInRange {
    #[primary_span]
    #[label(hir_typeck_missing_parentheses_in_range)]
    pub span: Span,
    pub ty_str: String,
    pub method_name: String,
    #[subdiagnostic]
    pub add_missing_parentheses: Option<AddMissingParenthesesInRange>,
}

#[derive(Subdiagnostic)]
#[multipart_suggestion(
    hir_typeck_add_missing_parentheses_in_range,
    style = "verbose",
    applicability = "maybe-incorrect"
)]
pub struct AddMissingParenthesesInRange {
    pub func_name: String,
    #[suggestion_part(code = "(")]
    pub left: Span,
    #[suggestion_part(code = ")")]
    pub right: Span,
}

pub struct TypeMismatchFruTypo {
    /// Span of the LHS of the range
    pub expr_span: Span,
    /// Span of the `..RHS` part of the range
    pub fru_span: Span,
    /// Rendered expression of the RHS of the range
    pub expr: Option<String>,
}

impl AddToDiagnostic for TypeMismatchFruTypo {
    fn add_to_diagnostic_with<G: EmissionGuarantee, F: SubdiagnosticMessageOp<G>>(
        self,
        diag: &mut DiagnosticBuilder<'_, G>,
        _f: F,
    ) {
        diag.arg("expr", self.expr.as_deref().unwrap_or("NONE"));

        // Only explain that `a ..b` is a range if it's split up
        if self.expr_span.between(self.fru_span).is_empty() {
            diag.span_note(self.expr_span.to(self.fru_span), fluent::hir_typeck_fru_note);
        } else {
            let mut multispan: MultiSpan = vec![self.expr_span, self.fru_span].into();
            multispan.push_span_label(self.expr_span, fluent::hir_typeck_fru_expr);
            multispan.push_span_label(self.fru_span, fluent::hir_typeck_fru_expr2);
            diag.span_note(multispan, fluent::hir_typeck_fru_note);
        }

        diag.span_suggestion(
            self.expr_span.shrink_to_hi(),
            fluent::hir_typeck_fru_suggestion,
            ", ",
            Applicability::MaybeIncorrect,
        );
    }
}

#[derive(LintDiagnostic)]
#[diag(hir_typeck_lossy_provenance_int2ptr)]
#[help]
pub struct LossyProvenanceInt2Ptr<'tcx> {
    pub expr_ty: Ty<'tcx>,
    pub cast_ty: Ty<'tcx>,
    #[subdiagnostic]
    pub sugg: LossyProvenanceInt2PtrSuggestion,
}

#[derive(Subdiagnostic)]
#[multipart_suggestion(hir_typeck_suggestion, applicability = "has-placeholders")]
pub struct LossyProvenanceInt2PtrSuggestion {
    #[suggestion_part(code = "(...).with_addr(")]
    pub lo: Span,
    #[suggestion_part(code = ")")]
    pub hi: Span,
}

#[derive(LintDiagnostic)]
#[diag(hir_typeck_lossy_provenance_ptr2int)]
#[help]
pub struct LossyProvenancePtr2Int<'tcx> {
    pub expr_ty: Ty<'tcx>,
    pub cast_ty: Ty<'tcx>,
    #[subdiagnostic]
    pub sugg: LossyProvenancePtr2IntSuggestion<'tcx>,
}

#[derive(Subdiagnostic)]
pub enum LossyProvenancePtr2IntSuggestion<'tcx> {
    #[multipart_suggestion(hir_typeck_suggestion, applicability = "maybe-incorrect")]
    NeedsParensCast {
        #[suggestion_part(code = "(")]
        expr_span: Span,
        #[suggestion_part(code = ").addr() as {cast_ty}")]
        cast_span: Span,
        cast_ty: Ty<'tcx>,
    },
    #[multipart_suggestion(hir_typeck_suggestion, applicability = "maybe-incorrect")]
    NeedsParens {
        #[suggestion_part(code = "(")]
        expr_span: Span,
        #[suggestion_part(code = ").addr()")]
        cast_span: Span,
    },
    #[suggestion(
        hir_typeck_suggestion,
        code = ".addr() as {cast_ty}",
        applicability = "maybe-incorrect"
    )]
    NeedsCast {
        #[primary_span]
        cast_span: Span,
        cast_ty: Ty<'tcx>,
    },
    #[suggestion(hir_typeck_suggestion, code = ".addr()", applicability = "maybe-incorrect")]
    Other {
        #[primary_span]
        cast_span: Span,
    },
}

#[derive(Subdiagnostic)]
pub enum HelpUseLatestEdition {
    #[help(hir_typeck_help_set_edition_cargo)]
    #[note(hir_typeck_note_edition_guide)]
    Cargo { edition: Edition },
    #[help(hir_typeck_help_set_edition_standalone)]
    #[note(hir_typeck_note_edition_guide)]
    Standalone { edition: Edition },
}

impl HelpUseLatestEdition {
    pub fn new() -> Self {
        let edition = LATEST_STABLE_EDITION;
        if rustc_session::utils::was_invoked_from_cargo() {
            Self::Cargo { edition }
        } else {
            Self::Standalone { edition }
        }
    }
}

#[derive(Diagnostic)]
#[diag(hir_typeck_invalid_callee, code = E0618)]
pub struct InvalidCallee {
    #[primary_span]
    pub span: Span,
    pub ty: String,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_int_to_fat, code = E0606)]
pub struct IntToWide<'tcx> {
    #[primary_span]
    #[label(hir_typeck_int_to_fat_label)]
    pub span: Span,
    pub metadata: &'tcx str,
    pub expr_ty: String,
    pub cast_ty: Ty<'tcx>,
    #[label(hir_typeck_int_to_fat_label_nightly)]
    pub expr_if_nightly: Option<Span>,
    pub known_wide: bool,
}

#[derive(Subdiagnostic)]
pub enum OptionResultRefMismatch {
    #[suggestion(
        hir_typeck_option_result_copied,
        code = ".copied()",
        style = "verbose",
        applicability = "machine-applicable"
    )]
    Copied {
        #[primary_span]
        span: Span,
        def_path: String,
    },
    #[suggestion(
        hir_typeck_option_result_cloned,
        code = ".cloned()",
        style = "verbose",
        applicability = "machine-applicable"
    )]
    Cloned {
        #[primary_span]
        span: Span,
        def_path: String,
    },
    // FIXME: #114050
    // #[suggestion(
    //     hir_typeck_option_result_asref,
    //     code = ".as_ref()",
    //     style = "verbose",
    //     applicability = "machine-applicable"
    // )]
    // AsRef {
    //     #[primary_span]
    //     span: Span,
    //     def_path: String,
    //     expected_ty: Ty<'tcx>,
    //     expr_ty: Ty<'tcx>,
    // },
}

pub struct RemoveSemiForCoerce {
    pub expr: Span,
    pub ret: Span,
    pub semi: Span,
}

impl AddToDiagnostic for RemoveSemiForCoerce {
    fn add_to_diagnostic_with<G: EmissionGuarantee, F: SubdiagnosticMessageOp<G>>(
        self,
        diag: &mut DiagnosticBuilder<'_, G>,
        _f: F,
    ) {
        let mut multispan: MultiSpan = self.semi.into();
        multispan.push_span_label(self.expr, fluent::hir_typeck_remove_semi_for_coerce_expr);
        multispan.push_span_label(self.ret, fluent::hir_typeck_remove_semi_for_coerce_ret);
        multispan.push_span_label(self.semi, fluent::hir_typeck_remove_semi_for_coerce_semi);
        diag.span_note(multispan, fluent::hir_typeck_remove_semi_for_coerce);

        diag.tool_only_span_suggestion(
            self.semi,
            fluent::hir_typeck_remove_semi_for_coerce_suggestion,
            "",
            Applicability::MaybeIncorrect,
        );
    }
}

#[derive(Diagnostic)]
#[diag(hir_typeck_const_select_must_be_const)]
#[help]
pub struct ConstSelectMustBeConst {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_const_select_must_be_fn)]
#[note]
#[help]
pub struct ConstSelectMustBeFn<'a> {
    #[primary_span]
    pub span: Span,
    pub ty: Ty<'a>,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_union_pat_multiple_fields)]
pub struct UnionPatMultipleFields {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_union_pat_dotdot)]
pub struct UnionPatDotDot {
    #[primary_span]
    pub span: Span,
}

#[derive(Subdiagnostic)]
#[multipart_suggestion(
    hir_typeck_use_is_empty,
    applicability = "maybe-incorrect",
    style = "verbose"
)]
pub struct UseIsEmpty {
    #[suggestion_part(code = "!")]
    pub lo: Span,
    #[suggestion_part(code = ".is_empty()")]
    pub hi: Span,
    pub expr_ty: String,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_arg_mismatch_indeterminate)]
pub struct ArgMismatchIndeterminate {
    #[primary_span]
    pub span: Span,
}

#[derive(Subdiagnostic)]
pub enum SuggestBoxing {
    #[note(hir_typeck_suggest_boxing_note)]
    #[multipart_suggestion(
        hir_typeck_suggest_boxing_when_appropriate,
        applicability = "machine-applicable"
    )]
    Unit {
        #[suggestion_part(code = "Box::new(())")]
        start: Span,
        #[suggestion_part(code = "")]
        end: Span,
    },
    #[note(hir_typeck_suggest_boxing_note)]
    AsyncBody,
    #[note(hir_typeck_suggest_boxing_note)]
    #[multipart_suggestion(
        hir_typeck_suggest_boxing_when_appropriate,
        applicability = "machine-applicable"
    )]
    Other {
        #[suggestion_part(code = "Box::new(")]
        start: Span,
        #[suggestion_part(code = ")")]
        end: Span,
    },
}

#[derive(Subdiagnostic)]
#[suggestion(
    hir_typeck_suggest_ptr_null_mut,
    applicability = "maybe-incorrect",
    code = "core::ptr::null_mut()"
)]
pub struct SuggestPtrNullMut {
    #[primary_span]
    pub span: Span,
}

#[derive(LintDiagnostic)]
#[diag(hir_typeck_trivial_cast)]
#[help]
pub struct TrivialCast<'tcx> {
    pub numeric: bool,
    pub expr_ty: Ty<'tcx>,
    pub cast_ty: Ty<'tcx>,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_no_associated_item, code = E0599)]
pub struct NoAssociatedItem {
    #[primary_span]
    pub span: Span,
    pub item_kind: &'static str,
    pub item_name: Ident,
    pub ty_prefix: Cow<'static, str>,
    pub ty_str: String,
    pub trait_missing_method: bool,
}

#[derive(Subdiagnostic)]
#[note(hir_typeck_candidate_trait_note)]
pub struct CandidateTraitNote {
    #[primary_span]
    pub span: Span,
    pub trait_name: String,
    pub item_name: Ident,
    pub action_or_ty: String,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_cannot_cast_to_bool, code = E0054)]
pub struct CannotCastToBool<'tcx> {
    #[primary_span]
    pub span: Span,
    pub expr_ty: Ty<'tcx>,
    #[subdiagnostic]
    pub help: CannotCastToBoolHelp,
}

#[derive(LintDiagnostic)]
#[diag(hir_typeck_cast_enum_drop)]
pub struct CastEnumDrop<'tcx> {
    pub expr_ty: Ty<'tcx>,
    pub cast_ty: Ty<'tcx>,
}

#[derive(Diagnostic)]
#[diag(hir_typeck_cast_unknown_pointer, code = E0641)]
pub struct CastUnknownPointer {
    #[primary_span]
    pub span: Span,
    pub to: bool,
    #[subdiagnostic]
    pub sub: CastUnknownPointerSub,
}

pub enum CastUnknownPointerSub {
    To(Span),
    From(Span),
}

impl rustc_errors::AddToDiagnostic for CastUnknownPointerSub {
    fn add_to_diagnostic_with<G: EmissionGuarantee, F: SubdiagnosticMessageOp<G>>(
        self,
        diag: &mut DiagnosticBuilder<'_, G>,
        f: F,
    ) {
        match self {
            CastUnknownPointerSub::To(span) => {
                let msg = f(diag, crate::fluent_generated::hir_typeck_label_to);
                diag.span_label(span, msg);
                let msg = f(diag, crate::fluent_generated::hir_typeck_note);
                diag.note(msg);
            }
            CastUnknownPointerSub::From(span) => {
                let msg = f(diag, crate::fluent_generated::hir_typeck_label_from);
                diag.span_label(span, msg);
            }
        }
    }
}

#[derive(Subdiagnostic)]
pub enum CannotCastToBoolHelp {
    #[suggestion(
        hir_typeck_suggestion,
        applicability = "machine-applicable",
        code = " != 0",
        style = "verbose"
    )]
    Numeric(#[primary_span] Span),
    #[label(hir_typeck_label)]
    Unsupported(#[primary_span] Span),
}

#[derive(Diagnostic)]
#[diag(hir_typeck_ctor_is_private, code = E0603)]
pub struct CtorIsPrivate {
    #[primary_span]
    pub span: Span,
    pub def: String,
}

#[derive(Subdiagnostic)]
#[note(hir_typeck_deref_is_empty)]
pub struct DerefImplsIsEmpty {
    #[primary_span]
    pub span: Span,
    pub deref_ty: String,
}

#[derive(Subdiagnostic)]
#[multipart_suggestion(
    hir_typeck_convert_using_method,
    applicability = "machine-applicable",
    style = "verbose"
)]
pub struct SuggestConvertViaMethod<'tcx> {
    #[suggestion_part(code = "{sugg}")]
    pub span: Span,
    #[suggestion_part(code = "")]
    pub borrow_removal_span: Option<Span>,
    pub sugg: String,
    pub expected: Ty<'tcx>,
    pub found: Ty<'tcx>,
}
