mod conf;
mod cmd;

fn main() {
    let conf: conf::Config = conf::Config::new();

    println!("{:?} {:?} {:?}", conf.github_conf.github_ref, conf.input_conf.username, conf.current_tag);

    cmd::test();
}
