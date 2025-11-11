use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "curl")]
pub struct Opt{
    #[structopt(name = "URL")]
    pub url: String,

    #[structopt(short = "X", default_value = "GET")]
    pub method: String,

    #[structopt(short = "d")]
    pub data: Option<String>,

    #[structopt(long = "json")]
    pub json: Option<String>,
}

