const express = require('express');
const { GearApi, ProgramMetadata, GearKeyring } = require('@gear-js/api');
const fs = require('fs');

const PORT = 8085;
const providerAddress = 'wss://testnet.vara.network';
const programId = '0x08300feba7072f43c2f0140ef2f25b9b5a1d0cbe66534861d958ca8c86e21a21';
const AccountMnemonic = 'account family aisle flavor ketchup inch pelican mountain tube advance peanut panel';
const metaFile = 'amber_msgs.meta.txt';

const app = express();
app.use(express.json());

let gearApi;
async function initGearApi() {
  gearApi = await GearApi.create({ providerAddress });
}

initGearApi();

// update message in contract
app.post('/api/data', async (req, res) => {
  const data = req.body;
  const keyring = await GearKeyring.fromMnemonic(AccountMnemonic, 'Amber');

  try {
    const metadata = fs.readFileSync('amber_msgs.meta.txt');
    const meta = ProgramMetadata.from(`0x${metadata}`);

    const gas = await gearApi.program.calculateGas.handle(
      programId,
      programId,
      { cmd: 0, content: data.content },
      0,
      false,
      meta,
    );
    console.log("gas.toHuman()", gas.toHuman());

    let gasLimitX2 = gas.min_limit * 2;
    console.log("gasLimitX2", gasLimitX2);
    
    const message = {
      destination: programId,
      payload: { cmd: 0, content: data.content },
      gasLimit: gasLimitX2,
      value:  1000000000000,
    };

    let extrinsic = await gearApi.message.send(message, meta);
    let headersSent = false;
    try {
      await extrinsic.signAndSend(keyring, (event) => {
        const txHash = event.txHash.toHuman();
        if (!headersSent) {
          res.status(201).json({ status: "ok", tx: txHash });
        }
        headersSent = true;
      });
    } catch (error) {
      res.status(500).json({ error: 'Error' });
      console.error(`${error.name}: ${error.message}`);
    }
  } catch (error) {
    console.error(`${error.name}: ${error.message}`);
    res.status(500).json({ error: 'Error' });
  }
});

// return message from contract
app.get('/api/data', async (req, res) => {
  try {
      const metadata = fs.readFileSync(metaFile);
      const meta = ProgramMetadata.from(`0x${metadata}`);

      try {
        const programState = await gearApi.programState.read(
          { programId }, meta, meta.types.state.output
        );

        let stateData = programState.toHuman();
        console.log('programState', stateData);

        res.json({ status: "ok", message: stateData });
      } catch (error) {
        console.error('Error reading program state:', error);
        res.status(500).json({ error: 'Error reading program state' });
      }
  } catch (error) {
    console.error('Error meta:', error);
    res.status(500).json({ error: 'Error reading program meta' });
  }
});

app.listen(PORT, () => {
  console.log(`Server is running on port ${PORT}`);
});