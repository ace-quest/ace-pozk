// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const { ethers, upgrades, network } = require("hardhat");
const { writeFile, readFileSync } = require('fs');

// zytron testnet
// const VK_1 = "0xDeb08b8247b866ff05856ce4883Dcd23F5E35adA";
// const VK_2 = "0x33682F75895E986546A09D60F7ef5Ee6a53383d8";
// const VERIFIER = "0x17c3Aef40495c2fcC9bc1880AeAAAf455fDfA5bE";
// const SHUFFLE = "0xbC9b4e9d43830f747e65873A5e122DDd9C9d769b";

// base sepolia
const VK_1 = "0xf55fB7932ca0179ad0a18307AC5cd87bd7A8c61E";
const VK_2 = "0x499094F6Ac3C351EF2d113f2851b7F1b7e761B09";
const VERIFIER = "0x1DD1253e7F245a763776a94A886FB3ED1FEed01b";
const SHUFFLE = "0x40C2cAc8cD71FB82B2B3b72Ae24d797fE904FcE1";

async function deployContract(name, params=[]) {
  const Factory = await ethers.getContractFactory(name);
  const contract = await Factory.deploy(...params);
  const address = await contract.getAddress();
  console.log(`${name} address: ${address}`);

  return address;
}

async function deployContractWithProxy(name, params=[]) {
  const Factory = await ethers.getContractFactory(name);
  const contract = await upgrades.deployProxy(Factory, params);
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  console.log(`${name} address: ${address}`);

  return address;
}

async function deploy() {
  const vk_1 = await deployContract("VerifierKey_52_1", []);
  const vk_2 = await deployContract("VerifierKey_52_2", []);

  const shuffle17Verifier = await deployContract("Shuffle17Verifier", [vk_1, vk_2]);
  const shuffle17 = await deployContractWithProxy("Shuffle17", [shuffle17Verifier]);
}

async function upgrade() {
  console.log(`Shuffle52 upgrading`);
  const shuffle52Verifier = await deployContract("Shuffle52Verifier", [VK_1, VK_2]);
  const C2 = await ethers.getContractFactory("Shuffle17");
  const prover2 = await C2.attach(SHUFFLE);
  await prover2.setVerifier(shuffle52Verifier);
  console.log(`Shuffle52 upgraded`);
}

async function test() {
  const C2 = await ethers.getContractFactory("Shuffle17Verifier");
  const prover2 = await C2.attach(VERIFIER);

  const publics_file = `../test/test_publics`;
  const proof_file = `../test/test_proof`;
  const publics = readFileSync(publics_file, 'utf8');
  const proof = readFileSync(proof_file, 'utf8');
  const res  = await prover2.verify(publics, proof);
  console.log("Shuffle verify:", res);
}

async function main() {
  // await deploy();
  // await upgrade();
  await test();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
