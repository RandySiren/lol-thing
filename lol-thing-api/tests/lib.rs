use lol_thing_api::LolThingApi;

#[cfg(test)]

mod test {

    use league_client_connector::RiotLockFile;

    #[test]
    fn test_new_lolthingapi() {
        let lockfile: RiotLockFile = RiotLockFile {
            address: String::from("address"),
            b64_auth: String::from("b64_auth"),
            password: String::from("password"),
            pid: 0,
            port: 0,
            process: String::from("process"),
            protocol: String::from("protocol"),
            username: String::from("username"),
        };

        let lol_thing_api = lol_thing_api::LolThingApi::new(lockfile);

        assert_eq!(lol_thing_api.lockfile, lockfile);
    }
}
