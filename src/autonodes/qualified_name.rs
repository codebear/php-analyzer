use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::namespace_name_as_prefix::NamespaceNameAsPrefixNode;
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
pub enum QualifiedNameChildren {
    Name(Box<NameNode>),
    NamespaceNameAsPrefix(Box<NamespaceNameAsPrefixNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl QualifiedNameChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                QualifiedNameChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => QualifiedNameChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => QualifiedNameChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "name" => QualifiedNameChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "namespace_name_as_prefix" => QualifiedNameChildren::NamespaceNameAsPrefix(Box::new(
                NamespaceNameAsPrefixNode::parse(node, source)?,
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
                QualifiedNameChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => QualifiedNameChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => QualifiedNameChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "name" => QualifiedNameChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "namespace_name_as_prefix" => QualifiedNameChildren::NamespaceNameAsPrefix(Box::new(
                NamespaceNameAsPrefixNode::parse(node, source)?,
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
            QualifiedNameChildren::Comment(x) => x.get_utype(state, emitter),
            QualifiedNameChildren::TextInterpolation(x) => x.get_utype(state, emitter),
            QualifiedNameChildren::Error(x) => x.get_utype(state, emitter),
            QualifiedNameChildren::Name(x) => x.get_utype(state, emitter),
            QualifiedNameChildren::NamespaceNameAsPrefix(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            QualifiedNameChildren::Comment(x) => x.get_php_value(state, emitter),
            QualifiedNameChildren::TextInterpolation(x) => x.get_php_value(state, emitter),
            QualifiedNameChildren::Error(x) => x.get_php_value(state, emitter),
            QualifiedNameChildren::Name(x) => x.get_php_value(state, emitter),
            QualifiedNameChildren::NamespaceNameAsPrefix(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            QualifiedNameChildren::Comment(x) => x.read_from(state, emitter),
            QualifiedNameChildren::TextInterpolation(x) => x.read_from(state, emitter),
            QualifiedNameChildren::Error(x) => x.read_from(state, emitter),
            QualifiedNameChildren::Name(x) => x.read_from(state, emitter),
            QualifiedNameChildren::NamespaceNameAsPrefix(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for QualifiedNameChildren {
    fn brief_desc(&self) -> String {
        match self {
            QualifiedNameChildren::Comment(x) => {
                format!("QualifiedNameChildren::comment({})", x.brief_desc())
            }
            QualifiedNameChildren::TextInterpolation(x) => format!(
                "QualifiedNameChildren::text_interpolation({})",
                x.brief_desc()
            ),
            QualifiedNameChildren::Error(x) => {
                format!("QualifiedNameChildren::ERROR({})", x.brief_desc())
            }
            QualifiedNameChildren::Name(x) => {
                format!("QualifiedNameChildren::name({})", x.brief_desc())
            }
            QualifiedNameChildren::NamespaceNameAsPrefix(x) => format!(
                "QualifiedNameChildren::namespace_name_as_prefix({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            QualifiedNameChildren::Comment(x) => x.as_any(),
            QualifiedNameChildren::TextInterpolation(x) => x.as_any(),
            QualifiedNameChildren::Error(x) => x.as_any(),
            QualifiedNameChildren::Name(x) => x.as_any(),
            QualifiedNameChildren::NamespaceNameAsPrefix(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            QualifiedNameChildren::Comment(x) => x.children_any(),
            QualifiedNameChildren::TextInterpolation(x) => x.children_any(),
            QualifiedNameChildren::Error(x) => x.children_any(),
            QualifiedNameChildren::Name(x) => x.children_any(),
            QualifiedNameChildren::NamespaceNameAsPrefix(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            QualifiedNameChildren::Comment(x) => x.range(),
            QualifiedNameChildren::TextInterpolation(x) => x.range(),
            QualifiedNameChildren::Error(x) => x.range(),
            QualifiedNameChildren::Name(x) => x.range(),
            QualifiedNameChildren::NamespaceNameAsPrefix(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct QualifiedNameNode {
    pub range: Range,
    pub children: Vec<Box<QualifiedNameChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl QualifiedNameNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "qualified_name" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [qualified_name] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: QualifiedNameChildren::parse_vec(
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
        "qualified_name"
    }
}

impl NodeAccess for QualifiedNameNode {
    fn brief_desc(&self) -> String {
        "QualifiedNameNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::QualifiedName(self)
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
