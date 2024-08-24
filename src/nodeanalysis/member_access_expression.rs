use std::sync::{Arc, RwLock};

use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        member_access_expression::{
            MemberAccessExpressionName, MemberAccessExpressionNode, MemberAccessExpressionObject,
        },
    },
    issue::{Issue, IssueEmitter, VoidEmitter},
    symboldata::class::{ClassMemberVisibility, ClassName, ClassType, PropertyData},
    symbols::{Name, Symbol},
    types::union::{DiscreteType, DiscretlyAccessedType, PHPType, UnionType},
    value::PHPValue,
};

use super::analysis::ThirdPassAnalyzeableNode;
use crate::autotree::NodeAccess;

impl MemberAccessExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        // FIXME:
        // * mark property as used
        // * check if we're in a context where we're allowed to read
        // crate::missing!("{}.read_from(..)", self.kind());

        self.name.read_from(state, emitter);
        self.object.read_from(state, emitter);
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        // PHP 8.1 brings readonly properties, which makes it more relevant to analyze something here
        // a private property with default-value or constructor initialization might also be approachable
        None
        //        crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        let data = self.get_property_data(state, emitter)?;
        // eprintln!("Property data: {:?}", *unlocked);
        if let Some(x) = data.declared_type {
            Some(x.clone())
        } else if let Some(dt) = data.comment_type {
            Some(dt.0.clone())
        } else if let ClassMemberVisibility::Private = data.visibility {
            // If the property is private, we can during the rest of the analysis make sure
            // that no other type is written to the property,
            // for now, let's just return the constructor type
            // FIXME make sure that this is a trustable
            data.constructor_type
        } else {
            None
        }
    }

    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<PHPType>,
        _value: Option<PHPValue>,
    ) {
        // FIXME
        // * mark property as written to
        // * gather statistics to inferr possible types of undefined/untyped variables
        if state.in_constructor() {
            if let Some(writable) = self.get_mut_property_data(state, emitter) {
                let mut property_data = writable.write().unwrap();
                // void
                if let Some(val_type) = val_type {
                    if let Some(t) = property_data.constructor_type.take() {
                        property_data.constructor_type =
                            Some(UnionType::from_pair(val_type, t).into())
                    } else {
                        property_data.constructor_type = Some(val_type);
                    }
                }
            } else {
                // FIXME if we accept this as a property declaration, we can achieve better type-analysis throughout the code, and
                // rather emit a local error here of missing declaration
            }
            // FIXME we could also store the value in constructor_value, but for now, we make no use of it,
            // because there are to many factors to determine if it's trustable or not
        }

        crate::missing!("MemberAccessExpressionNode.write_to(..)");
    }

    pub fn get_property_name(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<Name> {
        self.name.get_member_name(state, emitter)
    }

    pub fn get_class_name(&self, state: &mut AnalysisState) -> Option<ClassName> {
        self.object.get_class_name(state, &VoidEmitter::new())
    }

    pub fn get_property_data(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PropertyData> {
        let class_name = self.object.get_class_name(state, emitter)?;

        let class_data_handle = state.symbol_data.get_class(&class_name)?;
        let member_name = self.get_property_name(state, emitter)?;

        let cdata = class_data_handle.read().unwrap();
        if let ClassType::Class(c) = &*cdata {
            c.get_property(&member_name, state)
        } else {
            None
        }
    }

    pub fn get_mut_property_data(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<Arc<RwLock<PropertyData>>> {
        let class_name = self.object.get_class_name(state, emitter)?;

        let class_data_handle = state.symbol_data.get_class(&class_name)?;
        let member_name = self.get_property_name(state, emitter)?;

        let cdata = class_data_handle.read().unwrap();
        if let ClassType::Class(c) = &*cdata {
            c.get_mut_property(&member_name, state)
        } else {
            None
        }
    }

    pub fn get_symbols(&self, state: &mut AnalysisState) -> Option<Vec<Symbol>> {
        let types = self.get_utype(state, &VoidEmitter::new())?;
        let mut symbols: Vec<_> = vec![];
        for dtype in types.as_discrete_variants() {
            match dtype {
                DiscretlyAccessedType::Discrete(d) => symbols.push(d.into()),
                DiscretlyAccessedType::Intersection(_) => crate::missing!("What to do?"),
            }
        }

        Some(symbols)
    }
}

impl MemberAccessExpressionName {
    pub fn get_member_name(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<Name> {
        match self {
            MemberAccessExpressionName::_Expression(expr) => {
                match expr.get_php_value(state, emitter) {
                    Some(PHPValue::String(s)) => Some(Name::from(s)),
                    _ => crate::missing_none!("member_access_expr_name.get_member_name(..)"),
                }
            }
            MemberAccessExpressionName::DynamicVariableName(dvn) => {
                match dvn.get_php_value(state, emitter) {
                    Some(PHPValue::String(s)) => Some(Name::from(s)),
                    _ => crate::missing_none!("member_access_expr_name.get_member_name(..)"),
                }
            }

            MemberAccessExpressionName::Name(n) => Some(n.get_name()),
            MemberAccessExpressionName::VariableName(vn) => {
                match vn.get_php_value(state, emitter) {
                    Some(PHPValue::String(s)) => Some(Name::from(s)),
                    _ => crate::missing_none!("member_access_expr_name.get_member_name(..)"),
                }
            }

            MemberAccessExpressionName::Extra(_) => None,
        }
    }
}

impl ThirdPassAnalyzeableNode for MemberAccessExpressionNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        self.object.read_from(state, emitter);

        let maybe_class_name = self.object.get_class_name(state, emitter);

        let maybe_property_name = self.get_property_name(state, emitter);
        if maybe_property_name.is_some() {
        } else {
            // FIXME should emit unable to determine member-name on round three
            emitter.emit(Issue::IndeterminablePropertyName(
                self.pos(state),
                maybe_class_name.as_ref().map(|cname| cname.fq_name.clone()),
            ));
        }

        if let Some(cname) = &maybe_class_name {
            if let Some(cdata_handle) = state.symbol_data.get_class(cname) {
                if let Some(property_name) = maybe_property_name {
                    match &*(cdata_handle.read().unwrap()) {
                        ClassType::Class(c) => {
                            if let Some(_) = c.get_property(&property_name, state) {
                                // void
                            } else {
                                emitter.emit(Issue::UnknownProperty(
                                    self.pos(state),
                                    cname.fq_name.clone(),
                                    property_name,
                                ));
                            }
                        }
                        ClassType::None => emitter.emit(Issue::ParseAnomaly(
                            self.object.pos(state),
                            "ClassType::None on property acccess?".into(),
                        )),
                        ClassType::Interface(interface) => {
                            // FIXME den her bør også emittes dersom property_name e ukjent
                            emitter.emit(Issue::PropertyAccessOnInterfaceType(
                                self.pos(state),
                                interface.get_fq_name(),
                                Some(property_name),
                            ))
                        }
                        ClassType::Trait(_) => crate::missing!("property access on trait"),
                    }
                }
            } else {
                // let fqname: String = format!("{}", cname.get_fq_name());
                // eprintln!("BALLE2 Unknown class {}, {:?}", fqname, cname.get_fq_name());
                emitter.emit(Issue::UnknownClass(self.pos(state), cname.fq_name.clone()));
            }
        } else {
            emitter.emit(Issue::PropertyAccessOnUnknownType(
                self.pos(state),
                maybe_property_name,
            ));
            // FIXME This should emit unknown object-type or something on round three
        }

        self.analyze_third_pass_children(&self.as_any(), state, emitter, path)
    }
}

impl MemberAccessExpressionObject {
    pub fn get_class_name(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<ClassName> {
        match self {
            MemberAccessExpressionObject::ArrayCreationExpression(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::CastExpression(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::ClassConstantAccessExpression(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::DynamicVariableName(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::EncapsedString(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::FunctionCallExpression(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::Heredoc(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::MemberAccessExpression(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::MemberCallExpression(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::Name(n) => {
                Some(ClassName::new_with_analysis_state(&n.get_name(), state))
            }
            MemberAccessExpressionObject::NullsafeMemberAccessExpression(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::NullsafeMemberCallExpression(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::ParenthesizedExpression(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::QualifiedName(qn) => {
                let fq_name = qn.get_fq_name(state);
                Some(ClassName::new_with_names(
                    fq_name.get_name().unwrap_or_else(Name::new),
                    fq_name,
                ))
            }
            MemberAccessExpressionObject::ScopedCallExpression(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::ScopedPropertyAccessExpression(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::String(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::SubscriptExpression(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::VariableName(vn) => {
                let ctype = vn.get_utype(state, emitter)?;
                match ctype.single_type()? {
                    DiscreteType::Named(name, fq_name) => {
                        Some(ClassName::new_with_names(name, fq_name))
                    }
                    DiscreteType::Unknown => None,

                    t => crate::missing_none!(
                        "{}.get_class_name(..) har et objekt av typen {:?}",
                        self.kind(),
                        t
                    ),
                }
            }
            MemberAccessExpressionObject::Nowdoc(_) => {
                crate::missing_none!("{}.get_class_name(..)", self.kind())
            }
            MemberAccessExpressionObject::Extra(_) => None,
        }
    }
}
