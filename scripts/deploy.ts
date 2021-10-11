import * as path from "path";
import chalk from "chalk";
import { LocalTerra, MsgExecuteContract } from "@terra-money/terra.js";
import {
  toEncodedBinary,
  sendTransaction,
  storeCode,
  instantiateContract,
  queryNativeTokenBalance,
  queryTokenBalance,
} from "./helpers";

//----------------------------------------------------------------------------------------
// Variables
//----------------------------------------------------------------------------------------

const terra = new LocalTerra();
const deployer = terra.wallets.test1;

//----------------------------------------------------------------------------------------
// Setup
//----------------------------------------------------------------------------------------

async function setupTest() {
  // Step 1. Upload TerraSwap Token code
  process.stdout.write("Uploading TerraSwap Token code... ");

  const cw20CodeId = await storeCode(
    terra,
    deployer,
    path.resolve(__dirname, "../artifacts/terraswap_token.wasm")
  );

  console.log(chalk.green("Done!"), `${chalk.blue("codeId")}=${cw20CodeId}`);

  // Step 2. Instantiate TerraSwap Token contract
  process.stdout.write("Instantiating TerraSwap Token contract... ");

  const contractResult = await instantiateContract(terra, deployer, deployer, cw20CodeId, {
    // this comes from the contract being deployed
    count: 0
  });

  const contractAddr = contractResult.logs[0].events[0].attributes[3].value;

  console.log(chalk.green("Done!"), `${chalk.blue("contractAddress")}=${contractAddr}`);

}