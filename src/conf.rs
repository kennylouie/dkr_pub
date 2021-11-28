use std::env;

pub struct Config {
    pub github_conf: GithubConf,
    pub input_conf: InputConf,
    pub current_tag: String,
}

impl<'a, 'b> Config {
    pub fn new() -> Config {
        let mut i: InputConf = InputConf::new();
        i.get_from_env();

        let mut g: GithubConf = GithubConf::new();
        g.get_from_env();

        Config {
            github_conf: g,
            input_conf: i,
            current_tag: "".to_owned(),
        }
    }
}

const GITHUB_WORKFLOW: &str = "GITHUB_WORKFLOW";
const GITHUB_ACTION: &str = "GITHUB_ACTION";
const GITHUB_ACTOR: &str = "GITHUB_ACTOR";
const GITHUB_REPOSITORY: &str = "GITHUB_REPOSITORY";
const GITHUB_COMMIT: &str = "GITHUB_COMMIT";
const GITHUB_EVENTNAME: &str = "GITHUB_EVENTNAME";
const GITHUB_EVENTPATH: &str = "GITHUB_EVENTPATH";
const GITHUB_REF: &str = "GITHUB_REF";

#[derive(Debug)]
pub struct GithubConf {
    pub workflow: String,
    pub action: String,
    pub actor: String,
    pub repository: String,
    pub commit: String,
    pub eventname: String,
    pub eventpath: String,
    pub github_ref: String,
}

impl GithubConf {
    fn new() -> GithubConf {
        GithubConf {
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
pub struct InputConf {
    pub name: String,
    pub username: String,
    pub password: String,
    pub registry: String,
    pub tag: String,
    pub build: bool,
}

impl InputConf {
    fn new() -> InputConf {
        InputConf {
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
