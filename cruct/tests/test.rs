use cruct_proc::cruct;

#[cruct(path = "./test_config.toml")]
struct Test {
    #[field(name = "else")]
    something: String
}
