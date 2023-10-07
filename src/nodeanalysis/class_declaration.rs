use std::{
    ffi::OsString,
    os::unix::prelude::OsStrExt,
    sync::{Arc, RwLock},
};

use crate::{
    analysis::state::{AnalysisState, ClassState},
    autonodes::{
        any::AnyNodeRef,
        class_declaration::{
            ClassDeclarationChildren, ClassDeclarationModifier, ClassDeclarationNode,
        },
        class_interface_clause::ClassInterfaceClauseChildren,
    },
    autotree::NodeAccess,
    extra::ExtraChild,
    issue::{Issue, IssueEmitter},
    phpdoc::types::{PHPDocComment, PHPDocEntry},
    symboldata::{
        class::{ClassData, ClassModifier, ClassName, ClassType},
        FileLocation,
    },
    symbols::{FullyQualifiedName, Name},
    types::union::UnionType,
};

use super::{
    analysis::{FirstPassAnalyzeableNode, SecondPassAnalyzeableNode, ThirdPassAnalyzeableNode},
    class::{AnalysisOfClassBaseLikeNode, AnalysisOfDeclaredNameNode},
};
use crate::nodeanalysis::class::AnalysisOfClassLikeNode;

#[derive(Clone, Copy)]
pub enum InlineGenericSearchMode {
    Class,
    Extends,
    Implements,
}

impl ClassDeclarationNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        ()
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        None
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        None
    }

    fn get_class_name(&self, state: &mut AnalysisState) -> ClassName {
        let decl_class_name = self.get_declared_name();
        // new_with_analysis_state går nok via use-map, og deklarert klassenavn bør ikke det...
        let class_name =
            ClassName::new_with_analysis_state_without_aliasing(&decl_class_name, state);
        class_name
    }

    fn get_class_data(&self, state: &mut AnalysisState) -> Arc<RwLock<ClassType>> {
        let class_name = self.get_class_name(state);
        state.symbol_data.get_or_create_class(&class_name)
    }

    pub(crate) fn get_inline_generic_doc_comment(
        &self,
        mode: InlineGenericSearchMode,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<Vec<UnionType>> {
        //
        // This is a very suboptimal solution, but it will suffice
        //
        let mut start_search_pos = self.name.range.end_byte;

        let mut end_search_pos = self.body.range.start_byte;

        for child in &self.children {
            if child.range().start_byte < start_search_pos {
                continue;
            }

            match (mode, &**child) {
                (InlineGenericSearchMode::Class, ClassDeclarationChildren::BaseClause(b)) => {
                    end_search_pos = b.range().start_byte;
                    break;
                }
                (InlineGenericSearchMode::Extends, ClassDeclarationChildren::BaseClause(b)) => {
                    start_search_pos = b.range().end_byte;
                }
                (
                    InlineGenericSearchMode::Class,
                    ClassDeclarationChildren::ClassInterfaceClause(i),
                ) => {
                    // we'll never get here if there is an extends-clause
                    end_search_pos = i.range().start_byte;
                    break;
                }
                (
                    InlineGenericSearchMode::Extends,
                    ClassDeclarationChildren::ClassInterfaceClause(i),
                ) => {
                    end_search_pos = i.range().start_byte;
                    break;
                }

                (InlineGenericSearchMode::Implements, ClassDeclarationChildren::BaseClause(b)) => {
                    start_search_pos = b.range().end_byte;
                }
                (
                    InlineGenericSearchMode::Implements,
                    ClassDeclarationChildren::ClassInterfaceClause(i),
                ) => {
                    start_search_pos = i.range().end_byte;
                }

                (_, ClassDeclarationChildren::Extra(ExtraChild::Comment(_))) => todo!(),

                (_, ClassDeclarationChildren::Extra(_)) => (),
            }
        }

        for ex in &self.extras {
            if ex.range().start_byte <= start_search_pos {
                continue;
            }
            if ex.range().end_byte >= end_search_pos {
                continue;
            }
            if let ExtraChild::Comment(c) = &**ex {
                let (_utype_list, _range) =
                    PHPDocComment::parse_inline_generic(&c.get_raw(), &c.range(), state, emitter)?;
            }
        }

        // self.children
        None
    }
}

impl AnalysisOfDeclaredNameNode for ClassDeclarationNode {
    fn get_declared_name(&self) -> Name {
        self.name.get_name()
    }
}

impl AnalysisOfClassBaseLikeNode for ClassDeclarationNode {}

impl AnalysisOfClassLikeNode for ClassDeclarationNode {
    fn get_interfaces(&self, state: &mut AnalysisState) -> Option<Vec<FullyQualifiedName>> {
        let mut ifs = vec![];
        for any_intf_claus in self.named_children("class_interface_clause") {
            if let AnyNodeRef::ClassInterfaceClause(intf_claus) = any_intf_claus {
                for intf in &intf_claus.children {
                    match &**intf {
                        ClassInterfaceClauseChildren::Name(n) => {
                            ifs.push(state.get_fq_symbol_name_from_local_name(&n.get_name()))
                        }
                        ClassInterfaceClauseChildren::QualifiedName(qn) => {
                            ifs.push(qn.get_fq_name(state))
                        }
                        _ => (),
                    }
                }
            }
        }
        if ifs.len() > 0 {
            Some(ifs)
        } else {
            None
        }
    }
}

impl FirstPassAnalyzeableNode for ClassDeclarationNode {
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        let class_name = self.get_class_name(state);
        let base_name = self.get_declared_base_class_name(state, emitter);

        let mut modifier = if let Some(modifier) = &self.modifier {
            match &**modifier {
                ClassDeclarationModifier::AbstractModifier(_) => ClassModifier::Abstract,
                ClassDeclarationModifier::FinalModifier(_) => ClassModifier::Final,
                _ => ClassModifier::None,
            }
        } else {
            ClassModifier::None
        };

        let mut deprecated = None;

        let mut generic_templates = vec![];

        let mut phpdoc = None;

        let mut phpdoc_base_class_name = None;
        let mut phpdoc_interfaces = vec![];

        if let Some((raw_doc_comment, php_doc_range)) = state.last_doc_comment.clone() {
            match PHPDocComment::parse(&raw_doc_comment, &php_doc_range) {
                Ok(doc) => {
                    phpdoc = Some(doc.clone());

                    for entry in &doc.entries {
                        match entry {
                            PHPDocEntry::EmptyLine(_) => continue,
                            PHPDocEntry::Template(_, template, _) => {
                                // void
                                generic_templates.push(template.into());
                            }
                            PHPDocEntry::Deprecated(dep_range, desc) => {
                                // void
                                if let Some(_) = deprecated {
                                    emitter.emit(Issue::DuplicateDeclaration(
                                        state.pos_from_range(dep_range.clone()),
                                        "@deprecated".into(),
                                    ));
                                }
                                deprecated = Some(if let Some(d) = desc {
                                    d.clone()
                                } else {
                                    OsString::new()
                                });
                            }
                            PHPDocEntry::Author(_, _) => {
                                // void
                            }
                            PHPDocEntry::Anything(_, _) => {
                                // void
                            }
                            PHPDocEntry::Description(_, _) => {
                                // void
                            }

                            PHPDocEntry::Todo(_, _) => {
                                // void
                            }
                            PHPDocEntry::Copyright(_, _) => {
                                // void
                            }
                            PHPDocEntry::Version(_, _) => {
                                // void
                            }
                            PHPDocEntry::See(_, _, _) => {
                                // void
                            }

                            PHPDocEntry::Abstract(range) => {
                                if modifier == ClassModifier::None {
                                    modifier = ClassModifier::Abstract;
                                    // FIXME emit hint to declare class as abstract for real
                                } else if modifier == ClassModifier::Abstract {
                                    emitter.emit(Issue::RedundantPHPDocEntry(
                                        state.pos_from_range(range.clone()),
                                        "Class is already declared abstract".into(),
                                    ));
                                } else if modifier == ClassModifier::Final {
                                    emitter.emit(Issue::InvalidPHPDocEntry(
                                        state.pos_from_range(range.clone()),
                                        "Can't declare a final class as abstract".into(),
                                    ));
                                }
                                // void
                            }
                            PHPDocEntry::GeneralWithParam(range, param, data) => {
                                // @extends is an alias for @inherits
                                let lcparam = param.to_ascii_lowercase();
                                let pname_u8v = lcparam.as_bytes();
                                match pname_u8v {
                                    b"extends" | b"implements" | b"inherits" | b"mixin" => {
                                        if let Some(ptype) = UnionType::parse(
                                            data.clone(),
                                            range.clone(),
                                            state,
                                            emitter,
                                        ) {
                                            match pname_u8v {
                                                b"extends" | b"inherits" => {
                                                    // TODO might emit
                                                    if let None = phpdoc_base_class_name {
                                                        if let Some(dtype) = ptype.single_type() {
                                                            phpdoc_base_class_name = Some(dtype);
                                                        } else {
                                                            emitter.emit(
                                                                Issue::InvalidPHPDocEntry(
                                                                    state.pos_from_range(
                                                                        range.clone(),
                                                                    ),
                                                                    "Can't @extend a union-type"
                                                                        .into(),
                                                                ),
                                                            );
                                                        }
                                                    } else {
                                                        emitter.emit(Issue::InvalidPHPDocEntry(
                                                            state.pos_from_range(range.clone()),
                                                            "Duplicate @extends-entry".into(),
                                                        ));
                                                    }
                                                }
                                                b"implements" => {
                                                    let mut types: Vec<_> =
                                                        ptype.types.into_iter().collect();
                                                    phpdoc_interfaces.append(&mut types);
                                                }
                                                b"mixin" => {
                                                    crate::missing!("Implement @mixin support")
                                                }
                                                _ => (),
                                            }
                                        } else {
                                            // void
                                            crate::missing!(
                                                "emit unhandledable @{} DATA: {:?}",
                                                param.to_string_lossy(),
                                                data
                                            );
                                        }
                                    }
                                    b"suppress" => {
                                        // void
                                    }
                                    _ => {
                                        let sparam: &str = &param.to_string_lossy();
                                        if !state.config.phpdoc.known_tags.contains(&sparam) {
                                            emitter.emit(Issue::UnknownPHPDocEntry(
                                                state.pos_from_range(range.clone()),
                                                format!(
                                                    "Unknown PHPDoc-entry @{}",
                                                    param.to_string_lossy()
                                                )
                                                .into(),
                                            ))
                                        }
                                    }
                                }

                                // void
                            }
                            PHPDocEntry::General(range, param) => {
                                // void
                                // let lcparam = param.to_ascii_lowercase();
                                let sparam: &str = &param.to_string_lossy();
                                if !state.config.phpdoc.known_tags.contains(&sparam) {
                                    emitter.emit(Issue::UnknownPHPDocEntry(
                                        state.pos_from_range(range.clone()),
                                        format!(
                                            "Unknown PHPDoc-entry @{}",
                                            param.to_string_lossy()
                                        )
                                        .into(),
                                    ));
                                }
                            }
                            _ => {
                                todo!(
                                    "E in {:?}:{} {:?}",
                                    state.filename,
                                    php_doc_range.start_point.row,
                                    entry
                                );
                            }
                        }
                    }
                }
                Err(_) => emitter.emit(Issue::PHPDocParseError(
                    state.pos_from_range(php_doc_range.clone()),
                )),
            }
        }

        if let Some(inline_class_name_generic) =
            self.get_inline_generic_doc_comment(InlineGenericSearchMode::Class, state, emitter)
        {
            todo!("NOE: {:#?}", inline_class_name_generic);
        }

        let interfaces = self.get_interfaces(state);

        let mut class_data =
            ClassData::new(FileLocation::new(self.name.pos(state)), class_name.clone());
        class_data.modifier = modifier;
        if let Some(_) = &base_name {
            class_data.base_class_name = base_name;
        }
        class_data.deprecated = deprecated;
        if generic_templates.len() > 0 {
            class_data.generic_templates = Some(generic_templates);
        }
        class_data.phpdoc_base_class_name = phpdoc_base_class_name;
        class_data.phpdoc_interfaces = phpdoc_interfaces;
        if let Some(int) = interfaces {
            class_data.interfaces = int
                .iter()
                .map(|iname| {
                    ClassName::new_with_names(
                        iname.get_name().unwrap_or_else(|| Name::new()),
                        iname.clone(),
                    )
                })
                .collect();
        }
        class_data.phpdoc = phpdoc;

        let symbol_data = state.symbol_data.get_or_create_class(&class_name);
        {
            let mut unlocked = symbol_data.write().unwrap();
            match *unlocked {
                ClassType::None => {
                    *unlocked = ClassType::Class(class_data);
                }
                _ => {
                    emitter.emit(Issue::DuplicateClass(
                        self.pos(state),
                        class_name.get_fq_name().clone(),
                    ));
                    //                     emitter.emit(state.filename.as_ref(), self.range, format!("Duplicate class {:?}. Not analyzing interior.", class_name.get_fq_name()).into());
                    return;
                }
            }
        }
        // eprintln!("ClassDeclarationNode.analyze_round_one(): Analyzed os fram til {:?}", class_data);
        state.in_class = Some(ClassState::Class(class_name, symbol_data));
        state.last_doc_comment = None;
        self.analyze_first_pass_children(&self.as_any(), state, emitter);
        state.in_class = None;
    }
}

impl SecondPassAnalyzeableNode for ClassDeclarationNode {
    fn analyze_second_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        let class_name = self.get_class_name(state);
        state.in_class = Some(ClassState::Class(class_name, self.get_class_data(state)));
        self.analyze_second_pass_children(&self.as_any(), state, emitter);
        state.in_class = None;
    }
}

impl ThirdPassAnalyzeableNode for ClassDeclarationNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        let class_name = self.get_class_name(state);
        state.in_class = Some(ClassState::Class(class_name, self.get_class_data(state)));
        state.last_doc_comment = None;
        let res = self.analyze_third_pass_children(&self.as_any(), state, emitter, path);
        state.in_class = None;
        res
    }
}
