use league_client_connector::RiotLockFile;

pub struct LolThingApi {
    pub lockfile: RiotLockFile,
}

#[allow(unused_variables)]
impl LolThingApi {
    pub fn new(lockfile: RiotLockFile) -> Self {
        LolThingApi { lockfile }
    }
}
