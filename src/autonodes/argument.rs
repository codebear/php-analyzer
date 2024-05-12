use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::reference_modifier::ReferenceModifierNode;
use crate::autonodes::variadic_unpacking::VariadicUnpackingNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::parser::Range;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum ArgumentChildren {
    _Expression(Box<_ExpressionNode>),
    Name(Box<NameNode>),
    VariadicUnpacking(Box<VariadicUnpackingNode>),
    Extra(ExtraChild),
}

impl NodeParser for ArgumentChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ArgumentChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ArgumentChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "name" => ArgumentChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "variadic_unpacking" => ArgumentChildren::VariadicUnpacking(Box::new(
                VariadicUnpackingNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ArgumentChildren::_Expression(y))
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
}

impl ArgumentChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ArgumentChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ArgumentChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "name" => ArgumentChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "variadic_unpacking" => ArgumentChildren::VariadicUnpacking(Box::new(
                VariadicUnpackingNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ArgumentChildren::_Expression(y))
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
        match self {
            ArgumentChildren::Extra(y) => y.kind(),
            ArgumentChildren::_Expression(y) => y.kind(),
            ArgumentChildren::Name(y) => y.kind(),
            ArgumentChildren::VariadicUnpacking(y) => y.kind(),
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
            ArgumentChildren::Extra(x) => x.get_utype(state, emitter),
            ArgumentChildren::_Expression(x) => x.get_utype(state, emitter),
            ArgumentChildren::Name(x) => x.get_utype(state, emitter),
            ArgumentChildren::VariadicUnpacking(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ArgumentChildren::Extra(x) => x.get_php_value(state, emitter),
            ArgumentChildren::_Expression(x) => x.get_php_value(state, emitter),
            ArgumentChildren::Name(x) => x.get_php_value(state, emitter),
            ArgumentChildren::VariadicUnpacking(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ArgumentChildren::Extra(x) => x.read_from(state, emitter),
            ArgumentChildren::_Expression(x) => x.read_from(state, emitter),
            ArgumentChildren::Name(x) => x.read_from(state, emitter),
            ArgumentChildren::VariadicUnpacking(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ArgumentChildren {
    fn brief_desc(&self) -> String {
        match self {
            ArgumentChildren::Extra(x) => format!("ArgumentChildren::extra({})", x.brief_desc()),
            ArgumentChildren::_Expression(x) => {
                format!("ArgumentChildren::_expression({})", x.brief_desc())
            }
            ArgumentChildren::Name(x) => format!("ArgumentChildren::name({})", x.brief_desc()),
            ArgumentChildren::VariadicUnpacking(x) => {
                format!("ArgumentChildren::variadic_unpacking({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ArgumentChildren::Extra(x) => x.as_any(),
            ArgumentChildren::_Expression(x) => x.as_any(),
            ArgumentChildren::Name(x) => x.as_any(),
            ArgumentChildren::VariadicUnpacking(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ArgumentChildren::Extra(x) => x.children_any(),
            ArgumentChildren::_Expression(x) => x.children_any(),
            ArgumentChildren::Name(x) => x.children_any(),
            ArgumentChildren::VariadicUnpacking(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ArgumentChildren::Extra(x) => x.range(),
            ArgumentChildren::_Expression(x) => x.range(),
            ArgumentChildren::Name(x) => x.range(),
            ArgumentChildren::VariadicUnpacking(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArgumentNode {
    pub range: Range,
    pub name: Option<NameNode>,
    pub reference_modifier: Option<ReferenceModifierNode>,
    pub child: Box<ArgumentChildren>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ArgumentNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "argument" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [argument] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let name: Option<NameNode> = Result::from(
            node.parse_child("name", source)
                .mark_skipped_node(&mut skip_nodes)
                .into(),
        )?;
        let reference_modifier: Option<ReferenceModifierNode> = Result::from(
            node.parse_child("reference_modifier", source)
                .mark_skipped_node(&mut skip_nodes)
                .into(),
        )?;
        Ok(Self {
            range,
            name,
            reference_modifier,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| !skip_nodes.contains(&node.id()))
                .filter(|node| node.kind() != "comment")
                .map(|k| ArgumentChildren::parse(k, source))
                .collect::<Result<Vec<ArgumentChildren>, ParseError>>()?
                .drain(..)
                .map(|j| Box::new(j))
                .next()
                .expect("Should be a child"),
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment")
                    .filter(|node| !skip_nodes.contains(&node.id())),
                source,
            )?,
        })
    }
}

impl ArgumentNode {
    pub fn kind(&self) -> &'static str {
        "argument"
    }
}

impl NodeAccess for ArgumentNode {
    fn brief_desc(&self) -> String {
        "ArgumentNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::Argument(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.name {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.reference_modifier {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.child.as_any());
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
