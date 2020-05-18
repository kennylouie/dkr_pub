use std::env;

const GITHUB_WORKFLOW: &str = "GITHUB_WORKFLOW";
const GITHUB_ACTION: &str = "GITHUB_ACTION";
const GITHUB_ACTOR: &str = "GITHUB_ACTOR";
const GITHUB_REPOSITORY: &str = "GITHUB_REPOSITORY";
const GITHUB_COMMIT: &str = "GITHUB_COMMIT";
const GITHUB_EVENTNAME: &str = "GITHUB_EVENTNAME";
const GITHUB_EVENTPATH: &str = "GITHUB_EVENTPATH";
const GITHUB_REF: &str = "GITHUB_REF";

#[derive(Debug)]
struct Github {
    workflow: String,
    action: String,
    actor: String,
    repository: String,
    commit: String,
    eventname: String,
    eventpath: String,
    github_ref: String,
}

impl Github {
    fn new() -> Github {
        Github {
            workflow: "".to_owned(),
            action: "".to_owned(),
            actor: "".to_owned(),
            repository: "".to_owned(),
            commit: "".to_owned(),
            eventname: "".to_owned(),
            eventpath: "".to_owned(),
            github_ref: "".to_owned(),
        }
    }

    fn get_from_env(&mut self) {
        let empty = |_| "".to_owned();

        self.workflow = env::var(GITHUB_WORKFLOW)
            .unwrap_or_else(empty);

        self.action = env::var(GITHUB_ACTION)
            .unwrap_or_else(empty);

        self.actor = env::var(GITHUB_ACTOR)
            .unwrap_or_else(empty);

        self.repository = env::var(GITHUB_REPOSITORY)
            .unwrap_or_else(empty);

        self.commit = env::var(GITHUB_COMMIT)
            .unwrap_or_else(empty);

        self.eventname = env::var(GITHUB_EVENTNAME)
            .unwrap_or_else(empty);

        self.eventpath = env::var(GITHUB_EVENTPATH)
            .unwrap_or_else(empty);

        self.github_ref = env::var(GITHUB_REF)
            .unwrap_or_else(empty);
    }
}

const INPUT_NAME: &str = "INPUT_NAME";
const INPUT_USERNAME: &str = "INPUT_USERNAME";
const INPUT_PASSWORD: &str = "INPUT_PASSWORD";
const INPUT_REGISTRY: &str = "INPUT_REGISTRY";
const INPUT_TAG: &str = "INPUT_TAG";
const INPUT_BUILD: &str = "INPUT_BUILD";

#[derive(Debug)]
struct Inputs {
    name: String,
    username: String,
    password: String,
    registry: String,
    tag: String,
    build: bool,
}

impl Inputs {
    fn new() -> Inputs {
        Inputs {
            name: "".to_owned(),
            username: "".to_owned(),
            password: "".to_owned(),
            registry: "".to_owned(),
            tag: "".to_owned(),
            build: true,
        }
    }

    fn get_from_env(&mut self) {
        let empty = |_| "".to_owned();

        self.name = env::var(INPUT_NAME)
            .unwrap_or_else(empty);

        self.username = env::var(INPUT_USERNAME)
            .unwrap_or_else(empty);

        self.password = env::var(INPUT_PASSWORD)
            .unwrap_or_else(empty);

        self.registry = env::var(INPUT_REGISTRY)
            .unwrap_or_else(empty);

        self.tag = env::var(INPUT_TAG)
            .unwrap_or_else(empty);

        self.build = env::var(INPUT_BUILD)
            .unwrap_or_else(empty).parse::<bool>().unwrap_or(true);
    }
}

struct Config<'a, 'b> {
    github_conf: &'a Github,
    input_conf: &'b Inputs,
    current_tag: &'a String,
}

impl<'a, 'b> Config<'a, 'b> {
    fn new(g: &'a Github, i: &'b Inputs) -> Config<'a, 'b> {
        Config {
            github_conf: g,
            input_conf: i,
            current_tag: &g.github_ref,
        }
    }
}

fn main() {
    let mut inputs: Inputs = Inputs::new();
    inputs.get_from_env();

    let mut github: Github = Github::new();
    github.get_from_env();

    let conf: Config = Config::new(&github, &inputs);

    println!("{:?} {:?} {:?}", conf.github_conf.github_ref, conf.input_conf.username, conf.current_tag);
}
