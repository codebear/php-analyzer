use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::class_interface_clause::ClassInterfaceClauseNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::enum_declaration_list::EnumDeclarationListNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::primitive_type::PrimitiveTypeNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum EnumDeclarationChildren {
    ClassInterfaceClause(Box<ClassInterfaceClauseNode>),
    PrimitiveType(Box<PrimitiveTypeNode>),
    Extra(ExtraChild),
}

impl NodeParser for EnumDeclarationChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => EnumDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => EnumDeclarationChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => EnumDeclarationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "class_interface_clause" => EnumDeclarationChildren::ClassInterfaceClause(Box::new(
                ClassInterfaceClauseNode::parse(node, source)?,
            )),
            "primitive_type" => EnumDeclarationChildren::PrimitiveType(Box::new(
                PrimitiveTypeNode::parse(node, source)?,
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

impl EnumDeclarationChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => EnumDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => EnumDeclarationChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => EnumDeclarationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "class_interface_clause" => EnumDeclarationChildren::ClassInterfaceClause(Box::new(
                ClassInterfaceClauseNode::parse(node, source)?,
            )),
            "primitive_type" => EnumDeclarationChildren::PrimitiveType(Box::new(
                PrimitiveTypeNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            EnumDeclarationChildren::Extra(y) => y.kind(),
            EnumDeclarationChildren::ClassInterfaceClause(y) => y.kind(),
            EnumDeclarationChildren::PrimitiveType(y) => y.kind(),
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
            EnumDeclarationChildren::Extra(x) => x.get_utype(state, emitter),
            EnumDeclarationChildren::ClassInterfaceClause(x) => x.get_utype(state, emitter),
            EnumDeclarationChildren::PrimitiveType(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            EnumDeclarationChildren::Extra(x) => x.get_php_value(state, emitter),
            EnumDeclarationChildren::ClassInterfaceClause(x) => x.get_php_value(state, emitter),
            EnumDeclarationChildren::PrimitiveType(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            EnumDeclarationChildren::Extra(x) => x.read_from(state, emitter),
            EnumDeclarationChildren::ClassInterfaceClause(x) => x.read_from(state, emitter),
            EnumDeclarationChildren::PrimitiveType(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for EnumDeclarationChildren {
    fn brief_desc(&self) -> String {
        match self {
            EnumDeclarationChildren::Extra(x) => {
                format!("EnumDeclarationChildren::extra({})", x.brief_desc())
            }
            EnumDeclarationChildren::ClassInterfaceClause(x) => format!(
                "EnumDeclarationChildren::class_interface_clause({})",
                x.brief_desc()
            ),
            EnumDeclarationChildren::PrimitiveType(x) => format!(
                "EnumDeclarationChildren::primitive_type({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            EnumDeclarationChildren::Extra(x) => x.as_any(),
            EnumDeclarationChildren::ClassInterfaceClause(x) => x.as_any(),
            EnumDeclarationChildren::PrimitiveType(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            EnumDeclarationChildren::Extra(x) => x.children_any(),
            EnumDeclarationChildren::ClassInterfaceClause(x) => x.children_any(),
            EnumDeclarationChildren::PrimitiveType(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            EnumDeclarationChildren::Extra(x) => x.range(),
            EnumDeclarationChildren::ClassInterfaceClause(x) => x.range(),
            EnumDeclarationChildren::PrimitiveType(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumDeclarationNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub body: EnumDeclarationListNode,
    pub name: NameNode,
    pub children: Vec<Box<EnumDeclarationChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for EnumDeclarationNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "enum_declaration" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [enum_declaration] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let attributes: Option<AttributeListNode> = Result::from(
            node.parse_child("attributes", source)
                .mark_skipped_node(&mut skip_nodes)
                .into(),
        )?;
        let body: EnumDeclarationListNode = Result::from(
            node.parse_child("body", source)
                .mark_skipped_node(&mut skip_nodes)
                .into(),
        )?;
        let name: NameNode = Result::from(
            node.parse_child("name", source)
                .mark_skipped_node(&mut skip_nodes)
                .into(),
        )?;
        Ok(Self {
            range,
            attributes,
            body,
            name,
            children: EnumDeclarationChildren::parse_vec(
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
}

impl EnumDeclarationNode {
    pub fn kind(&self) -> &'static str {
        "enum_declaration"
    }
}

impl NodeAccess for EnumDeclarationNode {
    fn brief_desc(&self) -> String {
        "EnumDeclarationNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::EnumDeclaration(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.body.as_any());
        child_vec.push(self.name.as_any());
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
