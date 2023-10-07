use crate::analysis::state::AnalysisState;
use crate::autonodes::abstract_modifier::AbstractModifierNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::base_clause::BaseClauseNode;
use crate::autonodes::class_interface_clause::ClassInterfaceClauseNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::declaration_list::DeclarationListNode;
use crate::autonodes::final_modifier::FinalModifierNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::readonly_modifier::ReadonlyModifierNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
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
pub enum ClassDeclarationModifier {
    AbstractModifier(Box<AbstractModifierNode>),
    FinalModifier(Box<FinalModifierNode>),
    ReadonlyModifier(Box<ReadonlyModifierNode>),
    Extra(ExtraChild),
}

impl ClassDeclarationModifier {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ClassDeclarationModifier::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ClassDeclarationModifier::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ClassDeclarationModifier::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "abstract_modifier" => ClassDeclarationModifier::AbstractModifier(Box::new(
                AbstractModifierNode::parse(node, source)?,
            )),
            "final_modifier" => ClassDeclarationModifier::FinalModifier(Box::new(
                FinalModifierNode::parse(node, source)?,
            )),
            "readonly_modifier" => ClassDeclarationModifier::ReadonlyModifier(Box::new(
                ReadonlyModifierNode::parse(node, source)?,
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
            "comment" => ClassDeclarationModifier::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ClassDeclarationModifier::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ClassDeclarationModifier::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "abstract_modifier" => ClassDeclarationModifier::AbstractModifier(Box::new(
                AbstractModifierNode::parse(node, source)?,
            )),
            "final_modifier" => ClassDeclarationModifier::FinalModifier(Box::new(
                FinalModifierNode::parse(node, source)?,
            )),
            "readonly_modifier" => ClassDeclarationModifier::ReadonlyModifier(Box::new(
                ReadonlyModifierNode::parse(node, source)?,
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
            ClassDeclarationModifier::Extra(x) => x.get_utype(state, emitter),
            ClassDeclarationModifier::AbstractModifier(x) => x.get_utype(state, emitter),
            ClassDeclarationModifier::FinalModifier(x) => x.get_utype(state, emitter),
            ClassDeclarationModifier::ReadonlyModifier(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ClassDeclarationModifier::Extra(x) => x.get_php_value(state, emitter),
            ClassDeclarationModifier::AbstractModifier(x) => x.get_php_value(state, emitter),
            ClassDeclarationModifier::FinalModifier(x) => x.get_php_value(state, emitter),
            ClassDeclarationModifier::ReadonlyModifier(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ClassDeclarationModifier::Extra(x) => x.read_from(state, emitter),
            ClassDeclarationModifier::AbstractModifier(x) => x.read_from(state, emitter),
            ClassDeclarationModifier::FinalModifier(x) => x.read_from(state, emitter),
            ClassDeclarationModifier::ReadonlyModifier(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ClassDeclarationModifier {
    fn brief_desc(&self) -> String {
        match self {
            ClassDeclarationModifier::Extra(x) => {
                format!("ClassDeclarationModifier::extra({})", x.brief_desc())
            }
            ClassDeclarationModifier::AbstractModifier(x) => format!(
                "ClassDeclarationModifier::abstract_modifier({})",
                x.brief_desc()
            ),
            ClassDeclarationModifier::FinalModifier(x) => format!(
                "ClassDeclarationModifier::final_modifier({})",
                x.brief_desc()
            ),
            ClassDeclarationModifier::ReadonlyModifier(x) => format!(
                "ClassDeclarationModifier::readonly_modifier({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ClassDeclarationModifier::Extra(x) => x.as_any(),
            ClassDeclarationModifier::AbstractModifier(x) => x.as_any(),
            ClassDeclarationModifier::FinalModifier(x) => x.as_any(),
            ClassDeclarationModifier::ReadonlyModifier(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ClassDeclarationModifier::Extra(x) => x.children_any(),
            ClassDeclarationModifier::AbstractModifier(x) => x.children_any(),
            ClassDeclarationModifier::FinalModifier(x) => x.children_any(),
            ClassDeclarationModifier::ReadonlyModifier(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ClassDeclarationModifier::Extra(x) => x.range(),
            ClassDeclarationModifier::AbstractModifier(x) => x.range(),
            ClassDeclarationModifier::FinalModifier(x) => x.range(),
            ClassDeclarationModifier::ReadonlyModifier(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassDeclarationChildren {
    BaseClause(Box<BaseClauseNode>),
    ClassInterfaceClause(Box<ClassInterfaceClauseNode>),
    Extra(ExtraChild),
}

impl ClassDeclarationChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ClassDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ClassDeclarationChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ClassDeclarationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "base_clause" => {
                ClassDeclarationChildren::BaseClause(Box::new(BaseClauseNode::parse(node, source)?))
            }
            "class_interface_clause" => ClassDeclarationChildren::ClassInterfaceClause(Box::new(
                ClassInterfaceClauseNode::parse(node, source)?,
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
            "comment" => ClassDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ClassDeclarationChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ClassDeclarationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "base_clause" => {
                ClassDeclarationChildren::BaseClause(Box::new(BaseClauseNode::parse(node, source)?))
            }
            "class_interface_clause" => ClassDeclarationChildren::ClassInterfaceClause(Box::new(
                ClassInterfaceClauseNode::parse(node, source)?,
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
            ClassDeclarationChildren::Extra(x) => x.get_utype(state, emitter),
            ClassDeclarationChildren::BaseClause(x) => x.get_utype(state, emitter),
            ClassDeclarationChildren::ClassInterfaceClause(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ClassDeclarationChildren::Extra(x) => x.get_php_value(state, emitter),
            ClassDeclarationChildren::BaseClause(x) => x.get_php_value(state, emitter),
            ClassDeclarationChildren::ClassInterfaceClause(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ClassDeclarationChildren::Extra(x) => x.read_from(state, emitter),
            ClassDeclarationChildren::BaseClause(x) => x.read_from(state, emitter),
            ClassDeclarationChildren::ClassInterfaceClause(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ClassDeclarationChildren {
    fn brief_desc(&self) -> String {
        match self {
            ClassDeclarationChildren::Extra(x) => {
                format!("ClassDeclarationChildren::extra({})", x.brief_desc())
            }
            ClassDeclarationChildren::BaseClause(x) => {
                format!("ClassDeclarationChildren::base_clause({})", x.brief_desc())
            }
            ClassDeclarationChildren::ClassInterfaceClause(x) => format!(
                "ClassDeclarationChildren::class_interface_clause({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ClassDeclarationChildren::Extra(x) => x.as_any(),
            ClassDeclarationChildren::BaseClause(x) => x.as_any(),
            ClassDeclarationChildren::ClassInterfaceClause(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ClassDeclarationChildren::Extra(x) => x.children_any(),
            ClassDeclarationChildren::BaseClause(x) => x.children_any(),
            ClassDeclarationChildren::ClassInterfaceClause(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ClassDeclarationChildren::Extra(x) => x.range(),
            ClassDeclarationChildren::BaseClause(x) => x.range(),
            ClassDeclarationChildren::ClassInterfaceClause(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClassDeclarationNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub body: DeclarationListNode,
    pub modifier: Option<Box<ClassDeclarationModifier>>,
    pub name: NameNode,
    pub children: Vec<Box<ClassDeclarationChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl ClassDeclarationNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "class_declaration" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [class_declaration] on pos {}:{}",
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
        let body: DeclarationListNode = node
            .children_by_field_name("body", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| DeclarationListNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field body should exist");
        let modifier: Option<Box<ClassDeclarationModifier>> = node
            .children_by_field_name("modifier", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode2| ClassDeclarationModifier::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
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
        Ok(Self {
            range,
            attributes,
            body,
            modifier,
            name,
            children: ClassDeclarationChildren::parse_vec(
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
        "class_declaration"
    }
}

impl NodeAccess for ClassDeclarationNode {
    fn brief_desc(&self) -> String {
        "ClassDeclarationNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ClassDeclaration(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.body.as_any());
        if let Some(x) = &self.modifier {
            child_vec.push(x.as_any());
        }
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
