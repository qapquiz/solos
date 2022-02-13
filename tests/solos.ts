import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Solos } from '../target/types/solos';

describe('solos', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Solos as Program<Solos>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
