use crate::analysis::state::AnalysisState;
use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::declare_directive::DeclareDirectiveNode;
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
pub enum DeclareStatementChildren {
    _Statement(Box<_StatementNode>),
    DeclareDirective(Box<DeclareDirectiveNode>),
    Extra(ExtraChild),
}

impl DeclareStatementChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => DeclareStatementChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => DeclareStatementChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => DeclareStatementChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "declare_directive" => DeclareStatementChildren::DeclareDirective(Box::new(
                DeclareDirectiveNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _StatementNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| DeclareStatementChildren::_Statement(y))
                {
                    x
                } else {
                    return Err(ParseError::new(
                        node.range(),
                        format!("Parse error, unexpected node-type: {}", node.kind()),
                    ));
                }
            }
        })
    }

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => DeclareStatementChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => DeclareStatementChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => DeclareStatementChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "declare_directive" => DeclareStatementChildren::DeclareDirective(Box::new(
                DeclareDirectiveNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _StatementNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| DeclareStatementChildren::_Statement(y))
                    {
                        Some(x)
                    } else {
                        None
                    },
                )
            }
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
            DeclareStatementChildren::Extra(x) => x.get_utype(state, emitter),
            DeclareStatementChildren::_Statement(x) => x.get_utype(state, emitter),
            DeclareStatementChildren::DeclareDirective(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            DeclareStatementChildren::Extra(x) => x.get_php_value(state, emitter),
            DeclareStatementChildren::_Statement(x) => x.get_php_value(state, emitter),
            DeclareStatementChildren::DeclareDirective(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            DeclareStatementChildren::Extra(x) => x.read_from(state, emitter),
            DeclareStatementChildren::_Statement(x) => x.read_from(state, emitter),
            DeclareStatementChildren::DeclareDirective(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for DeclareStatementChildren {
    fn brief_desc(&self) -> String {
        match self {
            DeclareStatementChildren::Extra(x) => {
                format!("DeclareStatementChildren::extra({})", x.brief_desc())
            }
            DeclareStatementChildren::_Statement(x) => {
                format!("DeclareStatementChildren::_statement({})", x.brief_desc())
            }
            DeclareStatementChildren::DeclareDirective(x) => format!(
                "DeclareStatementChildren::declare_directive({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            DeclareStatementChildren::Extra(x) => x.as_any(),
            DeclareStatementChildren::_Statement(x) => x.as_any(),
            DeclareStatementChildren::DeclareDirective(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            DeclareStatementChildren::Extra(x) => x.children_any(),
            DeclareStatementChildren::_Statement(x) => x.children_any(),
            DeclareStatementChildren::DeclareDirective(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            DeclareStatementChildren::Extra(x) => x.range(),
            DeclareStatementChildren::_Statement(x) => x.range(),
            DeclareStatementChildren::DeclareDirective(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct DeclareStatementNode {
    pub range: Range,
    pub children: Vec<Box<DeclareStatementChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl DeclareStatementNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "declare_statement" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [declare_statement] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: DeclareStatementChildren::parse_vec(
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
        "declare_statement"
    }
}

impl NodeAccess for DeclareStatementNode {
    fn brief_desc(&self) -> String {
        "DeclareStatementNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::DeclareStatement(self)
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
