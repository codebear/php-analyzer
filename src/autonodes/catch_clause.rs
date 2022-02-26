use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::compound_statement::CompoundStatementNode;
use crate::autonodes::type_list::TypeListNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub struct CatchClauseNode {
    pub range: Range,
    pub body: CompoundStatementNode,
    pub name: Option<VariableNameNode>,
    pub type_: TypeListNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl CatchClauseNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "catch_clause" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [catch_clause] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let body: CompoundStatementNode = node
            .children_by_field_name("body", &mut node.walk())
            .map(|chnode1| CompoundStatementNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field body should exist");
        let name: Option<VariableNameNode> = node
            .children_by_field_name("name", &mut node.walk())
            .map(|chnode1| VariableNameNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let type_: TypeListNode = node
            .children_by_field_name("type", &mut node.walk())
            .map(|chnode1| TypeListNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field type should exist");
        Ok(Self {
            range,
            body,
            name,
            type_,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
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
        "catch_clause"
    }
}

impl NodeAccess for CatchClauseNode {
    fn brief_desc(&self) -> String {
        "CatchClauseNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::CatchClause(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.body.as_any());
        if let Some(x) = &self.name {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.type_.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
