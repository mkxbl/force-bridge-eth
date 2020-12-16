const fs = require("fs");
const TOML = require("@iarna/toml");

async function main() {
  const forceConfigPath = "/Users/vincent/.force-bridge/config.toml";
  const forceConfig = TOML.parse(fs.readFileSync(forceConfigPath));
  let network = forceConfig.networks_config[forceConfig.default_network];
  let accounts = network.ethereum_private_keys;
  const provider = new ethers.providers.JsonRpcProvider(
    network.ethereum_rpc_url
  );
  let sender = new ethers.Wallet("0x" + accounts[0], provider);
  let nonce = await sender.getTransactionCount();

  let promises = [];
  let addresses = [];
  const addition = 50;
  let i = 0;
  while (i < addition) {
    let wallet = ethers.Wallet.createRandom();
    let address = wallet.address;
    addresses.push(address);
    let value = ethers.utils.parseEther("1");
    let promise = await sender.sendTransaction({
      to: address,
      value: value,
      nonce: nonce,
    });
    promises.push(promise.wait(1));
    let private_key = wallet.privateKey.slice(2);
    accounts.push(private_key);
    nonce++;
    i++;
  }
  console.log(accounts);
  console.log(addresses);
  await Promise.all(promises);
  // for (const address of addresses) {
  //   let balance = await provider.getBalance(address);
  //   console.log(address, balance.toString());
  // }
  //
  // network.ethereum_private_keys = accounts;
  // const new_config = TOML.stringify(forceConfig);
  // fs.writeFileSync(forceConfigPath, new_config);
  // console.error("write eth addr into config successfully");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
