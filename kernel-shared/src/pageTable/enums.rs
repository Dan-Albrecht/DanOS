#[derive(PartialEq, Clone, Copy)]
pub enum Present {
    No,
    Yes,
}
#[derive(PartialEq, Clone, Copy)]
pub enum Writable {
    No,
    Yes,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Execute {
    No,
    Yes,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Cachable {
    No,
    Yes,
}

#[derive(PartialEq, Clone, Copy)]
pub enum WriteThrough {
    // Set both cache and memory at same time (slower)
    WriteTrough,
    // Set cache, eventually set memory (fast, might cause data loss)
    WriteBack,
}

#[derive(PartialEq, Clone, Copy)]
pub enum UserSupervisor {
    User,
    Supervisor,
}
