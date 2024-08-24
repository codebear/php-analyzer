use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::parser::Range;
use crate::types::union::PHPType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum ClassInterfaceClauseChildren {
    Name(Box<NameNode>),
    QualifiedName(Box<QualifiedNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for ClassInterfaceClauseChildren {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ClassInterfaceClauseChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                ClassInterfaceClauseChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => ClassInterfaceClauseChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "name" => ClassInterfaceClauseChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "qualified_name" => ClassInterfaceClauseChildren::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "ClassInterfaceClauseChildren: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl ClassInterfaceClauseChildren {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ClassInterfaceClauseChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                ClassInterfaceClauseChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => ClassInterfaceClauseChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "name" => ClassInterfaceClauseChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "qualified_name" => ClassInterfaceClauseChildren::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ClassInterfaceClauseChildren::Extra(y) => y.kind(),
            ClassInterfaceClauseChildren::Name(y) => y.kind(),
            ClassInterfaceClauseChildren::QualifiedName(y) => y.kind(),
        }
    }

    pub fn parse_vec<'a, I>(children: I, source: &[u8]) -> Result<Vec<Box<Self>>, ParseError>
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
    ) -> Option<PHPType> {
        match self {
            ClassInterfaceClauseChildren::Extra(x) => x.get_utype(state, emitter),
            ClassInterfaceClauseChildren::Name(x) => x.get_utype(state, emitter),
            ClassInterfaceClauseChildren::QualifiedName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ClassInterfaceClauseChildren::Extra(x) => x.get_php_value(state, emitter),
            ClassInterfaceClauseChildren::Name(x) => x.get_php_value(state, emitter),
            ClassInterfaceClauseChildren::QualifiedName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ClassInterfaceClauseChildren::Extra(x) => x.read_from(state, emitter),
            ClassInterfaceClauseChildren::Name(x) => x.read_from(state, emitter),
            ClassInterfaceClauseChildren::QualifiedName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ClassInterfaceClauseChildren {
    fn brief_desc(&self) -> String {
        match self {
            ClassInterfaceClauseChildren::Extra(x) => {
                format!("ClassInterfaceClauseChildren::extra({})", x.brief_desc())
            }
            ClassInterfaceClauseChildren::Name(x) => {
                format!("ClassInterfaceClauseChildren::name({})", x.brief_desc())
            }
            ClassInterfaceClauseChildren::QualifiedName(x) => format!(
                "ClassInterfaceClauseChildren::qualified_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            ClassInterfaceClauseChildren::Extra(x) => x.as_any(),
            ClassInterfaceClauseChildren::Name(x) => x.as_any(),
            ClassInterfaceClauseChildren::QualifiedName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            ClassInterfaceClauseChildren::Extra(x) => x.children_any(),
            ClassInterfaceClauseChildren::Name(x) => x.children_any(),
            ClassInterfaceClauseChildren::QualifiedName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ClassInterfaceClauseChildren::Extra(x) => x.range(),
            ClassInterfaceClauseChildren::Name(x) => x.range(),
            ClassInterfaceClauseChildren::QualifiedName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassInterfaceClauseNode {
    pub range: Range,
    pub children: Vec<Box<ClassInterfaceClauseChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ClassInterfaceClauseNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "class_interface_clause" {
            return Err(ParseError::new(range, format!("ClassInterfaceClauseNode: Node is of the wrong kind [{}] vs expected [class_interface_clause] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: ClassInterfaceClauseChildren::parse_vec(
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
}

impl ClassInterfaceClauseNode {
    pub fn kind(&self) -> &'static str {
        "class_interface_clause"
    }
}

impl NodeAccess for ClassInterfaceClauseNode {
    fn brief_desc(&self) -> String {
        "ClassInterfaceClauseNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::ClassInterfaceClause(self)
    }

    #[allow(clippy::vec_init_then_push)]
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
