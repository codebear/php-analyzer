use crate::analysis::state::AnalysisState;
use crate::autonodes::_type::_TypeNode;
use crate::autonodes::abstract_modifier::AbstractModifierNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::compound_statement::CompoundStatementNode;
use crate::autonodes::final_modifier::FinalModifierNode;
use crate::autonodes::formal_parameters::FormalParametersNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::reference_modifier::ReferenceModifierNode;
use crate::autonodes::static_modifier::StaticModifierNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::var_modifier::VarModifierNode;
use crate::autonodes::visibility_modifier::VisibilityModifierNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum MethodDeclarationChildren {
    AbstractModifier(Box<AbstractModifierNode>),
    FinalModifier(Box<FinalModifierNode>),
    StaticModifier(Box<StaticModifierNode>),
    VarModifier(Box<VarModifierNode>),
    VisibilityModifier(Box<VisibilityModifierNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl MethodDeclarationChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                MethodDeclarationChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => MethodDeclarationChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => MethodDeclarationChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "abstract_modifier" => MethodDeclarationChildren::AbstractModifier(Box::new(
                AbstractModifierNode::parse(node, source)?,
            )),
            "final_modifier" => MethodDeclarationChildren::FinalModifier(Box::new(
                FinalModifierNode::parse(node, source)?,
            )),
            "static_modifier" => MethodDeclarationChildren::StaticModifier(Box::new(
                StaticModifierNode::parse(node, source)?,
            )),
            "var_modifier" => MethodDeclarationChildren::VarModifier(Box::new(
                VarModifierNode::parse(node, source)?,
            )),
            "visibility_modifier" => MethodDeclarationChildren::VisibilityModifier(Box::new(
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

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => {
                MethodDeclarationChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => MethodDeclarationChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => MethodDeclarationChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "abstract_modifier" => MethodDeclarationChildren::AbstractModifier(Box::new(
                AbstractModifierNode::parse(node, source)?,
            )),
            "final_modifier" => MethodDeclarationChildren::FinalModifier(Box::new(
                FinalModifierNode::parse(node, source)?,
            )),
            "static_modifier" => MethodDeclarationChildren::StaticModifier(Box::new(
                StaticModifierNode::parse(node, source)?,
            )),
            "var_modifier" => MethodDeclarationChildren::VarModifier(Box::new(
                VarModifierNode::parse(node, source)?,
            )),
            "visibility_modifier" => MethodDeclarationChildren::VisibilityModifier(Box::new(
                VisibilityModifierNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        self.as_any().kind()
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
            MethodDeclarationChildren::Comment(x) => x.get_utype(state, emitter),
            MethodDeclarationChildren::TextInterpolation(x) => x.get_utype(state, emitter),
            MethodDeclarationChildren::Error(x) => x.get_utype(state, emitter),
            MethodDeclarationChildren::AbstractModifier(x) => x.get_utype(state, emitter),
            MethodDeclarationChildren::FinalModifier(x) => x.get_utype(state, emitter),
            MethodDeclarationChildren::StaticModifier(x) => x.get_utype(state, emitter),
            MethodDeclarationChildren::VarModifier(x) => x.get_utype(state, emitter),
            MethodDeclarationChildren::VisibilityModifier(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            MethodDeclarationChildren::Comment(x) => x.get_php_value(state, emitter),
            MethodDeclarationChildren::TextInterpolation(x) => x.get_php_value(state, emitter),
            MethodDeclarationChildren::Error(x) => x.get_php_value(state, emitter),
            MethodDeclarationChildren::AbstractModifier(x) => x.get_php_value(state, emitter),
            MethodDeclarationChildren::FinalModifier(x) => x.get_php_value(state, emitter),
            MethodDeclarationChildren::StaticModifier(x) => x.get_php_value(state, emitter),
            MethodDeclarationChildren::VarModifier(x) => x.get_php_value(state, emitter),
            MethodDeclarationChildren::VisibilityModifier(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            MethodDeclarationChildren::Comment(x) => x.read_from(state, emitter),
            MethodDeclarationChildren::TextInterpolation(x) => x.read_from(state, emitter),
            MethodDeclarationChildren::Error(x) => x.read_from(state, emitter),
            MethodDeclarationChildren::AbstractModifier(x) => x.read_from(state, emitter),
            MethodDeclarationChildren::FinalModifier(x) => x.read_from(state, emitter),
            MethodDeclarationChildren::StaticModifier(x) => x.read_from(state, emitter),
            MethodDeclarationChildren::VarModifier(x) => x.read_from(state, emitter),
            MethodDeclarationChildren::VisibilityModifier(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for MethodDeclarationChildren {
    fn brief_desc(&self) -> String {
        match self {
            MethodDeclarationChildren::Comment(x) => {
                format!("MethodDeclarationChildren::comment({})", x.brief_desc())
            }
            MethodDeclarationChildren::TextInterpolation(x) => format!(
                "MethodDeclarationChildren::text_interpolation({})",
                x.brief_desc()
            ),
            MethodDeclarationChildren::Error(x) => {
                format!("MethodDeclarationChildren::ERROR({})", x.brief_desc())
            }
            MethodDeclarationChildren::AbstractModifier(x) => format!(
                "MethodDeclarationChildren::abstract_modifier({})",
                x.brief_desc()
            ),
            MethodDeclarationChildren::FinalModifier(x) => format!(
                "MethodDeclarationChildren::final_modifier({})",
                x.brief_desc()
            ),
            MethodDeclarationChildren::StaticModifier(x) => format!(
                "MethodDeclarationChildren::static_modifier({})",
                x.brief_desc()
            ),
            MethodDeclarationChildren::VarModifier(x) => format!(
                "MethodDeclarationChildren::var_modifier({})",
                x.brief_desc()
            ),
            MethodDeclarationChildren::VisibilityModifier(x) => format!(
                "MethodDeclarationChildren::visibility_modifier({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            MethodDeclarationChildren::Comment(x) => x.as_any(),
            MethodDeclarationChildren::TextInterpolation(x) => x.as_any(),
            MethodDeclarationChildren::Error(x) => x.as_any(),
            MethodDeclarationChildren::AbstractModifier(x) => x.as_any(),
            MethodDeclarationChildren::FinalModifier(x) => x.as_any(),
            MethodDeclarationChildren::StaticModifier(x) => x.as_any(),
            MethodDeclarationChildren::VarModifier(x) => x.as_any(),
            MethodDeclarationChildren::VisibilityModifier(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            MethodDeclarationChildren::Comment(x) => x.children_any(),
            MethodDeclarationChildren::TextInterpolation(x) => x.children_any(),
            MethodDeclarationChildren::Error(x) => x.children_any(),
            MethodDeclarationChildren::AbstractModifier(x) => x.children_any(),
            MethodDeclarationChildren::FinalModifier(x) => x.children_any(),
            MethodDeclarationChildren::StaticModifier(x) => x.children_any(),
            MethodDeclarationChildren::VarModifier(x) => x.children_any(),
            MethodDeclarationChildren::VisibilityModifier(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            MethodDeclarationChildren::Comment(x) => x.range(),
            MethodDeclarationChildren::TextInterpolation(x) => x.range(),
            MethodDeclarationChildren::Error(x) => x.range(),
            MethodDeclarationChildren::AbstractModifier(x) => x.range(),
            MethodDeclarationChildren::FinalModifier(x) => x.range(),
            MethodDeclarationChildren::StaticModifier(x) => x.range(),
            MethodDeclarationChildren::VarModifier(x) => x.range(),
            MethodDeclarationChildren::VisibilityModifier(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MethodDeclarationNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub body: Option<CompoundStatementNode>,
    pub name: NameNode,
    pub parameters: FormalParametersNode,
    pub reference_modifier: Option<ReferenceModifierNode>,
    pub return_type: Option<_TypeNode>,
    pub children: Vec<Box<MethodDeclarationChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl MethodDeclarationNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "method_declaration" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [method_declaration] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let attributes: Option<AttributeListNode> = node
            .children_by_field_name("attributes", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| AttributeListNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let body: Option<CompoundStatementNode> = node
            .children_by_field_name("body", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| CompoundStatementNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let name: NameNode = node
            .children_by_field_name("name", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| NameNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field name should exist");
        let parameters: FormalParametersNode = node
            .children_by_field_name("parameters", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| FormalParametersNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field parameters should exist");
        let reference_modifier: Option<ReferenceModifierNode> = node
            .children_by_field_name("reference_modifier", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| ReferenceModifierNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let return_type: Option<_TypeNode> = node
            .children_by_field_name("return_type", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| _TypeNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        Ok(Self {
            range,
            attributes,
            body,
            name,
            parameters,
            reference_modifier,
            return_type,
            children: MethodDeclarationChildren::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| !skip_nodes.contains(&node.id()))
                    .filter(|node| node.kind() != "comment"),
                source,
            )?,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment")
                    .filter(|node| !skip_nodes.contains(&node.id())),
                source,
            )?,
        })
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            if child.kind() == "comment" {
                continue;
            }
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn kind(&self) -> &'static str {
        "method_declaration"
    }
}

impl NodeAccess for MethodDeclarationNode {
    fn brief_desc(&self) -> String {
        "MethodDeclarationNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::MethodDeclaration(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.body {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.name.as_any());
        child_vec.push(self.parameters.as_any());
        if let Some(x) = &self.reference_modifier {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.return_type {
            child_vec.push(x.as_any());
        }
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
