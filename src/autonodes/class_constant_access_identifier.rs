use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::name::NameNode;
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
pub enum ClassConstantAccessIdentifierChildren {
    _Expression(Box<_ExpressionNode>),
    Name(Box<NameNode>),
    Extra(ExtraChild),
}

impl NodeParser for ClassConstantAccessIdentifierChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ClassConstantAccessIdentifierChildren::Extra(ExtraChild::Comment(
                Box::new(CommentNode::parse(node, source)?),
            )),
            "ERROR" => ClassConstantAccessIdentifierChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "name" => ClassConstantAccessIdentifierChildren::Name(Box::new(NameNode::parse(
                node, source,
            )?)),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ClassConstantAccessIdentifierChildren::_Expression)
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

impl ClassConstantAccessIdentifierChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ClassConstantAccessIdentifierChildren::Extra(ExtraChild::Comment(
                Box::new(CommentNode::parse(node, source)?),
            )),
            "ERROR" => ClassConstantAccessIdentifierChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "name" => ClassConstantAccessIdentifierChildren::Name(Box::new(NameNode::parse(
                node, source,
            )?)),

            _ => {
                return Ok(_ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ClassConstantAccessIdentifierChildren::_Expression))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ClassConstantAccessIdentifierChildren::Extra(y) => y.kind(),
            ClassConstantAccessIdentifierChildren::_Expression(y) => y.kind(),
            ClassConstantAccessIdentifierChildren::Name(y) => y.kind(),
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
            ClassConstantAccessIdentifierChildren::Extra(x) => x.get_utype(state, emitter),
            ClassConstantAccessIdentifierChildren::_Expression(x) => x.get_utype(state, emitter),
            ClassConstantAccessIdentifierChildren::Name(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ClassConstantAccessIdentifierChildren::Extra(x) => x.get_php_value(state, emitter),
            ClassConstantAccessIdentifierChildren::_Expression(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessIdentifierChildren::Name(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ClassConstantAccessIdentifierChildren::Extra(x) => x.read_from(state, emitter),
            ClassConstantAccessIdentifierChildren::_Expression(x) => x.read_from(state, emitter),
            ClassConstantAccessIdentifierChildren::Name(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ClassConstantAccessIdentifierChildren {
    fn brief_desc(&self) -> String {
        match self {
            ClassConstantAccessIdentifierChildren::Extra(x) => format!(
                "ClassConstantAccessIdentifierChildren::extra({})",
                x.brief_desc()
            ),
            ClassConstantAccessIdentifierChildren::_Expression(x) => format!(
                "ClassConstantAccessIdentifierChildren::_expression({})",
                x.brief_desc()
            ),
            ClassConstantAccessIdentifierChildren::Name(x) => format!(
                "ClassConstantAccessIdentifierChildren::name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            ClassConstantAccessIdentifierChildren::Extra(x) => x.as_any(),
            ClassConstantAccessIdentifierChildren::_Expression(x) => x.as_any(),
            ClassConstantAccessIdentifierChildren::Name(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            ClassConstantAccessIdentifierChildren::Extra(x) => x.children_any(),
            ClassConstantAccessIdentifierChildren::_Expression(x) => x.children_any(),
            ClassConstantAccessIdentifierChildren::Name(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ClassConstantAccessIdentifierChildren::Extra(x) => x.range(),
            ClassConstantAccessIdentifierChildren::_Expression(x) => x.range(),
            ClassConstantAccessIdentifierChildren::Name(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassConstantAccessIdentifierNode {
    pub range: Range,
    pub child: Option<Box<ClassConstantAccessIdentifierChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ClassConstantAccessIdentifierNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "class_constant_access_identifier" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [class_constant_access_identifier] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| node.kind() != "comment")
                .map(|k| ClassConstantAccessIdentifierChildren::parse(k, source))
                .collect::<Result<Vec<ClassConstantAccessIdentifierChildren>, ParseError>>()?
                .drain(..)
                .map(Box::new)
                .next(),
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )?,
        })
    }
}

impl ClassConstantAccessIdentifierNode {
    pub fn kind(&self) -> &'static str {
        "class_constant_access_identifier"
    }
}

impl NodeAccess for ClassConstantAccessIdentifierNode {
    fn brief_desc(&self) -> String {
        "ClassConstantAccessIdentifierNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::ClassConstantAccessIdentifier(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.child {
            child_vec.push(x.as_any());
        }
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
