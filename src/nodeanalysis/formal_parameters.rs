use std::{collections::HashMap, ffi::OsString};

//use tree_sitter::Range;
use crate::parser::Range;
use crate::types::type_parser::TypeParser;
use crate::types::union::PHPType;
use crate::{
    analysis::state::AnalysisState,
    autonodes::formal_parameters::{FormalParametersChildren, FormalParametersNode},
    extra::ExtraChild,
    issue::IssueEmitter,
    phpdoc::types::{PHPDocComment, PHPDocEntry},
    symboldata::class::FunctionArgumentData,
    symbols::Name,
    types::union::{from_vec_parsed_type},
};

use crate::autotree::NodeAccess;

#[derive(Debug)]
enum ChildNode {
    ExtraNode(Box<ExtraChild>),
    ChildNode(Box<FormalParametersChildren>),
}

impl ChildNode {
    pub fn range(&self) -> Range {
        match self {
            ChildNode::ExtraNode(e) => e.range(),
            ChildNode::ChildNode(c) => c.range(),
        }
    }
}

impl FormalParametersNode {
    pub(crate) fn analyze_first_pass_parameters(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        param_map: &HashMap<Name, PHPDocEntry>,
        temp_generics: Option<&Vec<Name>>,
    ) -> Vec<FunctionArgumentData> {
        let mut params = vec![];
        let mut raw_comment: Option<(OsString, Range)> = None;
        let mut inline_phpdoc_type: Option<(Range, PHPType)> = None;
        let mut children: Vec<_> = self
            .children
            .iter()
            .map(|x| ChildNode::ChildNode(x.clone()))
            .collect();
        children.extend(self.extras.iter().map(|y| ChildNode::ExtraNode(y.clone())));
        children.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));

        // FIXME extract phpdoc-param-entries from function phpdoc

        for noe_child in &children {
            match noe_child {
                ChildNode::ExtraNode(extra) => match &**extra {
                    ExtraChild::Comment(c) => raw_comment = Some((c.get_raw(), c.range())),
                    ExtraChild::Error(_) => (),
                    ExtraChild::TextInterpolation(_) => (),
                },
                ChildNode::ChildNode(child) => match &**child {
                    FormalParametersChildren::PropertyPromotionParameter(_) => {
                        crate::missing!("PropertyPromotionParameter")
                    }
                    FormalParametersChildren::SimpleParameter(s) => {
                        let name = s.get_variable_name();
                        let arg_type = s.get_utype(state, emitter);
                        let default_value = s.get_default_value(state, emitter);

                        let mut vec = vec![b'$'];
                        vec.extend(name.as_bytes());
                        let vname = Name::from(vec);

                        let optional = false;
                        let nullable = false;
                        let phpdoc_entry = param_map.get(&vname).cloned();
                        let phpdoc_type =
                            if let Some(PHPDocEntry::Param(_range, types, _name, _desc)) =
                                &phpdoc_entry
                            {
                                from_vec_parsed_type(types.clone(), state, None, temp_generics)
                            } else {
                                None
                            };

                        let data = FunctionArgumentData {
                            name,
                            arg_type,
                            default_value,
                            nullable,
                            optional,
                            inline_phpdoc_type: inline_phpdoc_type.clone(),
                            phpdoc_entry,
                            phpdoc_type,
                            variadic: false,
                        };

                        params.push(data);
                    }
                    FormalParametersChildren::VariadicParameter(variadic) => {
                        let name = variadic.get_variable_name();
                        let arg_type = variadic.get_utype(state, emitter);
                        let default_value = None;
                        let nullable = false; // Hmm
                        let optional = true;

                        let mut vec = vec![b'$'];
                        vec.extend(name.as_bytes());
                        let vname = Name::from(vec);

                        let phpdoc_entry = param_map.get(&vname).cloned();
                        let phpdoc_type =
                            if let Some(PHPDocEntry::Param(_range, types, _name, _desc)) =
                                &phpdoc_entry
                            {
                                from_vec_parsed_type(types.clone(), state, None, temp_generics)
                            } else {
                                None
                            };

                        let data = FunctionArgumentData {
                            name,
                            arg_type,
                            default_value,
                            nullable,
                            optional,
                            inline_phpdoc_type: inline_phpdoc_type.clone(),
                            phpdoc_entry,
                            phpdoc_type,
                            variadic: true,
                        };

                        params.push(data);
                    }
                    FormalParametersChildren::Extra(ExtraChild::Comment(c)) => {
                        raw_comment = Some((c.get_raw(), c.range()));
                    }
                    FormalParametersChildren::Extra(_) => (),
                },
            }
            if inline_phpdoc_type.is_some() {
                raw_comment = None;
                inline_phpdoc_type = None;
            } else if let Some((raw, range)) = &raw_comment {
                inline_phpdoc_type = self.get_inline_doc_comment_type(raw, range, state, emitter);
            }
        }

        params
    }

    ///
    /// if someone has declared types with inline comments
    /// function foo(/** bool */ $arg)
    /// vi extract it as a regular phpdoc-entry
    pub fn get_inline_doc_comment_type(
        &self,
        raw: &OsString,
        range: &Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<(Range, PHPType)> {
        let phpdoc_entries = PHPDocComment::parse(raw, range).ok()?;
        if phpdoc_entries.entries.len() != 1 {
            return None;
        }
        let (range, content) =
            if let PHPDocEntry::Anything(range, content) = &phpdoc_entries.entries[0] {
                (range, content)
            } else {
                return None;
            };

        let utype = TypeParser::parse(content.clone(), *range, state, emitter)?;

        Some((*range, utype))
    }
}
