use structopt::StructOpt;

pub trait ICli {
    fn version() -> String {
        "0.0.1".into()
    }

    fn version_name() -> String {
        "simplifiable".into()
    }

    fn model(&self, mode: i8) -> String;
}

#[derive(Debug, StructOpt)]
#[structopt(name = "basic")]
pub struct BasicCmd {
    /// baseCmd subCommand
    #[structopt(subcommand)]
    pub sub_command: Option<SubCommand>,

    /// run node cmd
    #[structopt(flatten)]
    pub run: RunCmd,
}


impl ICli for BasicCmd {
    fn model(&self, mode: i8) -> String {
        match mode {
            0 => {
                "release".into()
            }
            1 => {
                "dev".into()
            }
            _ => {
                "test".into()
            }
        }
    }
}


// 定义子命令
#[derive(Debug, StructOpt)]
pub enum SubCommand {
    /// chain query info cmd
    #[structopt(name = "chainCmd")]
    ChainCmd(ChainCmd),

    /// net manager cmd
    #[structopt(name = "netCmd")]
    NetCmd(NetCmd),
}


#[derive(Debug, StructOpt)]
pub struct ChainCmd {
    /// print chain info
    pub info: Option<String>,
}

#[derive(Debug, StructOpt)]
pub struct NetCmd {
    /// print net info
    pub info: Option<String>,
}

// 默认命令选项
#[derive(Debug, StructOpt, Clone)]
pub struct RunCmd {
    /// default run port
    #[structopt(short, long, default_value = "7890")]
    port: isize,
    /// node run mode
    #[structopt(short, long, default_value = "dev")]
    mode: String,

}


pub fn run() {
    let r_cmd = BasicCmd::from_args();
    match &r_cmd.sub_command {
        Some(SubCommand::ChainCmd(chain_cmd)) => {
            println!("chain_cmd : {:?}", chain_cmd.info)
        }
        Some(SubCommand::NetCmd(net_cmd)) => {
            println!("net_cmd : {:?}", net_cmd.info)
        }
        None => {
            let run = r_cmd.run;
            println!("run cmd: {:?}", run.port)
        }
    }
}


#[cfg(test)]
mod test {
    use crate::command::run;

    #[test]
    fn test() {
        run()
    }
}