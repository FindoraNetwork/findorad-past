use clap::{ArgGroup, Parser};
use ruc::*;

use crate::entry::DevOperation;
use crate::node::start;

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("opt"))]
pub struct Command {
    #[clap(short = 'p',long, group = "opt")]
    pos: bool,

    #[clap(short = 's',long, group = "opt")]
    single: bool

}

impl Command {
    pub fn execute(&self) -> Result<()> {
        if self.single {
            let operation = DevOperation::Single(("./node".to_string(),"26657".to_string()));
            start(operation);
        }

        if self.pos {
            let path_port_vec = vec![
                ("./node/node1".to_string(),"26616".to_string()),
                ("./node/node2".to_string(),"26626".to_string()),
                ("./node/node3".to_string(),"26636".to_string()),
                ("./node/node4".to_string(),"26646".to_string()),
                ("./node/node5".to_string(),"26656".to_string()),
                ("./node/node6".to_string(),"26666".to_string()),
                ("./node/node7".to_string(),"26676".to_string()),
                ("./node/node8".to_string(),"26686".to_string()),
                ("./node/node9".to_string(),"26696".to_string()),
                ("./node/node10".to_string(),"26706".to_string()),
                ("./node/node11".to_string(),"26716".to_string()),
                ("./node/node12".to_string(),"26726".to_string()),
                ("./node/node13".to_string(),"26746".to_string()),
                ("./node/node14".to_string(),"26756".to_string()),
                ("./node/node15".to_string(),"26766".to_string()),
                ("./node/node16".to_string(),"26776".to_string()),
                ("./node/node17".to_string(),"26786".to_string()),
                ("./node/node18".to_string(),"26796".to_string()),
                ("./node/node19".to_string(),"26806".to_string()),
                ("./node/node20".to_string(),"26816".to_string()),
            ];

            let operation = DevOperation::Pos(path_port_vec);
            start(operation);
        }
        Ok(())
    }
}
