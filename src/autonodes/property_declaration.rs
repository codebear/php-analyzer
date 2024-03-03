use crate::analysis::state::AnalysisState;
use crate::autonodes::_type::_TypeNode;
use crate::autonodes::abstract_modifier::AbstractModifierNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::final_modifier::FinalModifierNode;
use crate::autonodes::property_element::PropertyElementNode;
use crate::autonodes::readonly_modifier::ReadonlyModifierNode;
use crate::autonodes::static_modifier::StaticModifierNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::var_modifier::VarModifierNode;
use crate::autonodes::visibility_modifier::VisibilityModifierNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::types::union::DiscreteType;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use std::ffi::OsStr;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum PropertyDeclarationModifiers {
    AbstractModifier(Box<AbstractModifierNode>),
    FinalModifier(Box<FinalModifierNode>),
    ReadonlyModifier(Box<ReadonlyModifierNode>),
    StaticModifier(Box<StaticModifierNode>),
    VarModifier(Box<VarModifierNode>),
    VisibilityModifier(Box<VisibilityModifierNode>),
    Extra(ExtraChild),
}

impl NodeParser for PropertyDeclarationModifiers {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => PropertyDeclarationModifiers::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                PropertyDeclarationModifiers::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => PropertyDeclarationModifiers::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "abstract_modifier" => PropertyDeclarationModifiers::AbstractModifier(Box::new(
                AbstractModifierNode::parse(node, source)?,
            )),
            "final_modifier" => PropertyDeclarationModifiers::FinalModifier(Box::new(
                FinalModifierNode::parse(node, source)?,
            )),
            "readonly_modifier" => PropertyDeclarationModifiers::ReadonlyModifier(Box::new(
                ReadonlyModifierNode::parse(node, source)?,
            )),
            "static_modifier" => PropertyDeclarationModifiers::StaticModifier(Box::new(
                StaticModifierNode::parse(node, source)?,
            )),
            "var_modifier" => PropertyDeclarationModifiers::VarModifier(Box::new(
                VarModifierNode::parse(node, source)?,
            )),
            "visibility_modifier" => PropertyDeclarationModifiers::VisibilityModifier(Box::new(
                VisibilityModifierNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
}

impl PropertyDeclarationModifiers {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => PropertyDeclarationModifiers::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                PropertyDeclarationModifiers::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => PropertyDeclarationModifiers::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "abstract_modifier" => PropertyDeclarationModifiers::AbstractModifier(Box::new(
                AbstractModifierNode::parse(node, source)?,
            )),
            "final_modifier" => PropertyDeclarationModifiers::FinalModifier(Box::new(
                FinalModifierNode::parse(node, source)?,
            )),
            "readonly_modifier" => PropertyDeclarationModifiers::ReadonlyModifier(Box::new(
                ReadonlyModifierNode::parse(node, source)?,
            )),
            "static_modifier" => PropertyDeclarationModifiers::StaticModifier(Box::new(
                StaticModifierNode::parse(node, source)?,
            )),
            "var_modifier" => PropertyDeclarationModifiers::VarModifier(Box::new(
                VarModifierNode::parse(node, source)?,
            )),
            "visibility_modifier" => PropertyDeclarationModifiers::VisibilityModifier(Box::new(
                VisibilityModifierNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            PropertyDeclarationModifiers::Extra(y) => y.kind(),
            PropertyDeclarationModifiers::AbstractModifier(y) => y.kind(),
            PropertyDeclarationModifiers::FinalModifier(y) => y.kind(),
            PropertyDeclarationModifiers::ReadonlyModifier(y) => y.kind(),
            PropertyDeclarationModifiers::StaticModifier(y) => y.kind(),
            PropertyDeclarationModifiers::VarModifier(y) => y.kind(),
            PropertyDeclarationModifiers::VisibilityModifier(y) => y.kind(),
        }
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match self {
            PropertyDeclarationModifiers::Extra(x) => x.get_utype(state, emitter),
            PropertyDeclarationModifiers::AbstractModifier(x) => x.get_utype(state, emitter),
            PropertyDeclarationModifiers::FinalModifier(x) => x.get_utype(state, emitter),
            PropertyDeclarationModifiers::ReadonlyModifier(x) => x.get_utype(state, emitter),
            PropertyDeclarationModifiers::StaticModifier(x) => x.get_utype(state, emitter),
            PropertyDeclarationModifiers::VarModifier(x) => x.get_utype(state, emitter),
            PropertyDeclarationModifiers::VisibilityModifier(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            PropertyDeclarationModifiers::Extra(x) => x.get_php_value(state, emitter),
            PropertyDeclarationModifiers::AbstractModifier(x) => x.get_php_value(state, emitter),
            PropertyDeclarationModifiers::FinalModifier(x) => x.get_php_value(state, emitter),
            PropertyDeclarationModifiers::ReadonlyModifier(x) => x.get_php_value(state, emitter),
            PropertyDeclarationModifiers::StaticModifier(x) => x.get_php_value(state, emitter),
            PropertyDeclarationModifiers::VarModifier(x) => x.get_php_value(state, emitter),
            PropertyDeclarationModifiers::VisibilityModifier(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            PropertyDeclarationModifiers::Extra(x) => x.read_from(state, emitter),
            PropertyDeclarationModifiers::AbstractModifier(x) => x.read_from(state, emitter),
            PropertyDeclarationModifiers::FinalModifier(x) => x.read_from(state, emitter),
            PropertyDeclarationModifiers::ReadonlyModifier(x) => x.read_from(state, emitter),
            PropertyDeclarationModifiers::StaticModifier(x) => x.read_from(state, emitter),
            PropertyDeclarationModifiers::VarModifier(x) => x.read_from(state, emitter),
            PropertyDeclarationModifiers::VisibilityModifier(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for PropertyDeclarationModifiers {
    fn brief_desc(&self) -> String {
        match self {
            PropertyDeclarationModifiers::Extra(x) => {
                format!("PropertyDeclarationModifiers::extra({})", x.brief_desc())
            }
            PropertyDeclarationModifiers::AbstractModifier(x) => format!(
                "PropertyDeclarationModifiers::abstract_modifier({})",
                x.brief_desc()
            ),
            PropertyDeclarationModifiers::FinalModifier(x) => format!(
                "PropertyDeclarationModifiers::final_modifier({})",
                x.brief_desc()
            ),
            PropertyDeclarationModifiers::ReadonlyModifier(x) => format!(
                "PropertyDeclarationModifiers::readonly_modifier({})",
                x.brief_desc()
            ),
            PropertyDeclarationModifiers::StaticModifier(x) => format!(
                "PropertyDeclarationModifiers::static_modifier({})",
                x.brief_desc()
            ),
            PropertyDeclarationModifiers::VarModifier(x) => format!(
                "PropertyDeclarationModifiers::var_modifier({})",
                x.brief_desc()
            ),
            PropertyDeclarationModifiers::VisibilityModifier(x) => format!(
                "PropertyDeclarationModifiers::visibility_modifier({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            PropertyDeclarationModifiers::Extra(x) => x.as_any(),
            PropertyDeclarationModifiers::AbstractModifier(x) => x.as_any(),
            PropertyDeclarationModifiers::FinalModifier(x) => x.as_any(),
            PropertyDeclarationModifiers::ReadonlyModifier(x) => x.as_any(),
            PropertyDeclarationModifiers::StaticModifier(x) => x.as_any(),
            PropertyDeclarationModifiers::VarModifier(x) => x.as_any(),
            PropertyDeclarationModifiers::VisibilityModifier(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            PropertyDeclarationModifiers::Extra(x) => x.children_any(),
            PropertyDeclarationModifiers::AbstractModifier(x) => x.children_any(),
            PropertyDeclarationModifiers::FinalModifier(x) => x.children_any(),
            PropertyDeclarationModifiers::ReadonlyModifier(x) => x.children_any(),
            PropertyDeclarationModifiers::StaticModifier(x) => x.children_any(),
            PropertyDeclarationModifiers::VarModifier(x) => x.children_any(),
            PropertyDeclarationModifiers::VisibilityModifier(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            PropertyDeclarationModifiers::Extra(x) => x.range(),
            PropertyDeclarationModifiers::AbstractModifier(x) => x.range(),
            PropertyDeclarationModifiers::FinalModifier(x) => x.range(),
            PropertyDeclarationModifiers::ReadonlyModifier(x) => x.range(),
            PropertyDeclarationModifiers::StaticModifier(x) => x.range(),
            PropertyDeclarationModifiers::VarModifier(x) => x.range(),
            PropertyDeclarationModifiers::VisibilityModifier(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PropertyDeclarationProperties {
    Comma(&'static str, Range),
    PropertyElement(Box<PropertyElementNode>),
    Extra(ExtraChild),
}

impl NodeParser for PropertyDeclarationProperties {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => PropertyDeclarationProperties::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                PropertyDeclarationProperties::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => PropertyDeclarationProperties::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "," => PropertyDeclarationProperties::Comma(",", node.range()),
            "property_element" => PropertyDeclarationProperties::PropertyElement(Box::new(
                PropertyElementNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
}

impl PropertyDeclarationProperties {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => PropertyDeclarationProperties::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                PropertyDeclarationProperties::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => PropertyDeclarationProperties::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "," => PropertyDeclarationProperties::Comma(",", node.range()),
            "property_element" => PropertyDeclarationProperties::PropertyElement(Box::new(
                PropertyElementNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            PropertyDeclarationProperties::Extra(y) => y.kind(),
            PropertyDeclarationProperties::Comma(y, _) => y,
            PropertyDeclarationProperties::PropertyElement(y) => y.kind(),
        }
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match self {
            PropertyDeclarationProperties::Extra(x) => x.get_utype(state, emitter),
            PropertyDeclarationProperties::Comma(_, _) => Some(DiscreteType::String.into()),
            PropertyDeclarationProperties::PropertyElement(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            PropertyDeclarationProperties::Extra(x) => x.get_php_value(state, emitter),
            PropertyDeclarationProperties::Comma(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            PropertyDeclarationProperties::PropertyElement(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            PropertyDeclarationProperties::Extra(x) => x.read_from(state, emitter),
            PropertyDeclarationProperties::Comma(_, _) => (),
            PropertyDeclarationProperties::PropertyElement(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for PropertyDeclarationProperties {
    fn brief_desc(&self) -> String {
        match self {
            PropertyDeclarationProperties::Extra(x) => {
                format!("PropertyDeclarationProperties::extra({})", x.brief_desc())
            }
            PropertyDeclarationProperties::Comma(a, _) => a.to_string(),
            PropertyDeclarationProperties::PropertyElement(x) => format!(
                "PropertyDeclarationProperties::property_element({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            PropertyDeclarationProperties::Extra(x) => x.as_any(),
            PropertyDeclarationProperties::Comma(a, b) => AnyNodeRef::StaticExpr(a, *b),
            PropertyDeclarationProperties::PropertyElement(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            PropertyDeclarationProperties::Extra(x) => x.children_any(),
            PropertyDeclarationProperties::Comma(_, _) => todo!("Crap"),
            PropertyDeclarationProperties::PropertyElement(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            PropertyDeclarationProperties::Extra(x) => x.range(),
            PropertyDeclarationProperties::Comma(_, r) => *r,
            PropertyDeclarationProperties::PropertyElement(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropertyDeclarationNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub modifiers: Vec<Box<PropertyDeclarationModifiers>>,
    pub properties: Vec<Box<PropertyDeclarationProperties>>,
    pub type_: Option<_TypeNode>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for PropertyDeclarationNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "property_declaration" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [property_declaration] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let attributes: Option<AttributeListNode> =
            Result::from(node.parse_child("attributes", source).into())?;
        let modifiers: Vec<Box<PropertyDeclarationModifiers>> =
            Result::from(node.parse_child("modifiers", source).into())?;
        let properties: Vec<Box<PropertyDeclarationProperties>> =
            Result::from(node.parse_child("properties", source).into())?;
        let type_: Option<_TypeNode> = Result::from(node.parse_child("type", source).into())?;
        Ok(Self {
            range,
            attributes,
            modifiers,
            properties,
            type_,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl PropertyDeclarationNode {
    pub fn kind(&self) -> &'static str {
        "property_declaration"
    }
}

impl NodeAccess for PropertyDeclarationNode {
    fn brief_desc(&self) -> String {
        "PropertyDeclarationNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::PropertyDeclaration(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
        child_vec.extend(self.modifiers.iter().map(|v| v.as_any()));
        child_vec.extend(self.properties.iter().map(|v| v.as_any()));
        if let Some(x) = &self.type_ {
            child_vec.push(x.as_any());
        }

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
