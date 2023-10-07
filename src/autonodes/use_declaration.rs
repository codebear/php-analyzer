use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::use_list::UseListNode;
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
pub enum UseDeclarationChildren {
    Name(Box<NameNode>),
    QualifiedName(Box<QualifiedNameNode>),
    UseList(Box<UseListNode>),
    Extra(ExtraChild),
}

impl UseDeclarationChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => UseDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UseDeclarationChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UseDeclarationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "name" => UseDeclarationChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "qualified_name" => UseDeclarationChildren::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "use_list" => {
                UseDeclarationChildren::UseList(Box::new(UseListNode::parse(node, source)?))
            }

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
            "comment" => UseDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UseDeclarationChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UseDeclarationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "name" => UseDeclarationChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "qualified_name" => UseDeclarationChildren::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "use_list" => {
                UseDeclarationChildren::UseList(Box::new(UseListNode::parse(node, source)?))
            }

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
            UseDeclarationChildren::Extra(x) => x.get_utype(state, emitter),
            UseDeclarationChildren::Name(x) => x.get_utype(state, emitter),
            UseDeclarationChildren::QualifiedName(x) => x.get_utype(state, emitter),
            UseDeclarationChildren::UseList(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            UseDeclarationChildren::Extra(x) => x.get_php_value(state, emitter),
            UseDeclarationChildren::Name(x) => x.get_php_value(state, emitter),
            UseDeclarationChildren::QualifiedName(x) => x.get_php_value(state, emitter),
            UseDeclarationChildren::UseList(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            UseDeclarationChildren::Extra(x) => x.read_from(state, emitter),
            UseDeclarationChildren::Name(x) => x.read_from(state, emitter),
            UseDeclarationChildren::QualifiedName(x) => x.read_from(state, emitter),
            UseDeclarationChildren::UseList(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for UseDeclarationChildren {
    fn brief_desc(&self) -> String {
        match self {
            UseDeclarationChildren::Extra(x) => {
                format!("UseDeclarationChildren::extra({})", x.brief_desc())
            }
            UseDeclarationChildren::Name(x) => {
                format!("UseDeclarationChildren::name({})", x.brief_desc())
            }
            UseDeclarationChildren::QualifiedName(x) => {
                format!("UseDeclarationChildren::qualified_name({})", x.brief_desc())
            }
            UseDeclarationChildren::UseList(x) => {
                format!("UseDeclarationChildren::use_list({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            UseDeclarationChildren::Extra(x) => x.as_any(),
            UseDeclarationChildren::Name(x) => x.as_any(),
            UseDeclarationChildren::QualifiedName(x) => x.as_any(),
            UseDeclarationChildren::UseList(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            UseDeclarationChildren::Extra(x) => x.children_any(),
            UseDeclarationChildren::Name(x) => x.children_any(),
            UseDeclarationChildren::QualifiedName(x) => x.children_any(),
            UseDeclarationChildren::UseList(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            UseDeclarationChildren::Extra(x) => x.range(),
            UseDeclarationChildren::Name(x) => x.range(),
            UseDeclarationChildren::QualifiedName(x) => x.range(),
            UseDeclarationChildren::UseList(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct UseDeclarationNode {
    pub range: Range,
    pub children: Vec<Box<UseDeclarationChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl UseDeclarationNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "use_declaration" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [use_declaration] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: UseDeclarationChildren::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() != "comment"),
                source,
            )?,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
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
        "use_declaration"
    }
}

impl NodeAccess for UseDeclarationNode {
    fn brief_desc(&self) -> String {
        "UseDeclarationNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::UseDeclaration(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
