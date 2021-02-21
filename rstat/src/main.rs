use std::path::PathBuf;
use structopt::StructOpt;
use rstat::srcstats::get_summary_src_stats;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    dir: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("The target directory is {:?}", opt.dir);
    let src_stats = get_summary_src_stats(
        opt.dir.as_path()).
        expect("fail to get the statistics of the input directory");
    println!("{}", src_stats);
}
