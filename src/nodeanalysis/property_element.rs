use std::sync::{Arc, RwLock};

use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        property_declaration::{PropertyDeclarationModifiers, PropertyDeclarationNode},
        property_element::PropertyElementNode,
    },
    autotree::NodeAccess,
    issue::{Issue, IssueEmitter},
    phpdoccomment::PHPDocComment,
    symboldata::{
        class::{ClassMemberVisibility, ClassModifier, PropertyData},
        FileLocation,
    },
    symbols::Name,
    types::union::UnionType,
};

impl PropertyElementNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        crate::missing!("{}.read_from(..)", self.kind());
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }

    pub fn get_property_name(&self) -> Name {
        self.name.get_variable_name()
    }

    pub fn get_property_data(
        &self,
        state: &mut AnalysisState,
    ) -> Option<Arc<RwLock<PropertyData>>> {
        let property_name = self.get_property_name();
        let class = if let Some(c) = &state.in_class {
            c
        } else {
            return None;
        };
        state
            .symbol_data
            .get_or_create_property(
                &class.get_name(),
                &property_name,
                FileLocation::new(self.pos(state)),
            )
            .clone()
    }

    pub fn get_doc_comment_type(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let (raw_doc_comment, doc_comment_range) = state.last_doc_comment.clone()?;

        let doc_comment = PHPDocComment::parse(&raw_doc_comment, doc_comment_range)?;

        let (type_str, range) = doc_comment.get_param("@var")?;

        UnionType::parse(type_str, range, state, emitter)
    }

    pub fn analyze_round_one_with_declaration(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        declaration: &PropertyDeclarationNode,
    ) {
        let data_handle = if let Some(handle) = self.get_property_data(state) {
            handle
        } else {
            emitter.emit(Issue::ParseAnomaly(
                self.pos(state),
                "Unable to get property data".into(),
            ));
            return;
        };

        let mut modifier = ClassModifier::None;
        let mut visibility = ClassMemberVisibility::Public;
        let mut is_static = false;
        for mods in &declaration.modifiers {
            match &**mods {
                PropertyDeclarationModifiers::AbstractModifier(_) => {
                    modifier = ClassModifier::Abstract;
                }
                PropertyDeclarationModifiers::FinalModifier(_) => {
                    modifier = ClassModifier::Final;
                }
                PropertyDeclarationModifiers::StaticModifier(_) => is_static = true,
                PropertyDeclarationModifiers::VarModifier(_) => {
                    visibility = ClassMemberVisibility::Public
                }
                PropertyDeclarationModifiers::VisibilityModifier(v) => {
                    visibility = v.get_visibility()
                }

                PropertyDeclarationModifiers::Comment(_)
                | PropertyDeclarationModifiers::TextInterpolation(_)
                | PropertyDeclarationModifiers::Error(_) => (),
            }
        }
        /*eprintln!(
            "declared type: {:?}",
            declaration
                .type_
                .as_ref()
                .and_then(|x| x.get_utype(state, emitter))
        );
        eprintln!(
            "phpdoc type: {:?}",
            self.get_doc_comment_type(state, emitter)
        );*/
        // FIXME se på doc-comment for type
        // FIXME doc_comment ska kanskje ha precedence fremfor declarated_type

        let declared_type = declaration
            .type_
            .as_ref()
            .and_then(|x| x.get_utype(state, emitter));

        let comment_type = self.get_doc_comment_type(state, emitter);

        /*let cname: String = if let Some(x) = &state.in_class {
            x.get_name().get_fq_name().to_string_lossy().to_string()
        } else {
            "".into()
        };
        eprintln!(
            "PROP: {}::${}: {:?} | {:?}",
            cname,
            self.get_property_name().to_string_lossy(),
            declared_type,
            comment_type
        );*/
        {
            let mut data = data_handle.write().unwrap();
            data.declared_type = declared_type;
            data.comment_type = comment_type;
            data.is_static = is_static;
            data.modifier = modifier;
            data.visibility = visibility;
        }
    }

    pub fn analyze_round_two_with_declaration(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        _declaration: &PropertyDeclarationNode,
    ) {
        // eprintln!("round two property declaration");
        let data_handle = if let Some(handle) = self.get_property_data(state) {
            handle
        } else {
            emitter.emit(Issue::ParseAnomaly(
                self.pos(state),
                "Unable to get property data".into(),
            ));
            return;
        };

        let mut data = data_handle.write().unwrap();
        data.default_value = if let Some(init) = &self.initializer {
            init.get_php_value(state, emitter)
        } else {
            None
        };
    }
}
