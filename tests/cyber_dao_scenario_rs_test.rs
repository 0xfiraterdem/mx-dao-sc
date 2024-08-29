use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/cyber-dao.mxsc.json", cyber_dao::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/cyber_dao.scen.json");
}
