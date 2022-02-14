import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Solos } from '../target/types/solos';
import { SolosStrategy } from '../target/types/solos_strategy';

describe('solos', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const solos = anchor.workspace.Solos as Program<Solos>;
  const solosStrategy = anchor.workspace.SolosStrategy as Program<SolosStrategy>;

  console.log("solos:", solos.programId.toBase58());
  console.log("solos-strategy:", solosStrategy.programId.toBase58());

  it('Is initialized!', async () => {
    // Add your test here.
    const solosStrategyInitTx = await solosStrategy.rpc.initialize({});
    console.log("solosStrategyInitTx transaction signature:", solosStrategyInitTx);
  });
});
