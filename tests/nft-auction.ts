import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftAuction } from "../target/types/nft_auction";

import {
  ConfirmOptions
} from "@solana/web3.js";

describe("NFT Auction House", () => {
  const connection = new anchor.web3.Connection(
    "http://127.0.0.1:8899",
    "confirmed"
  );

  const wallet = anchor.Wallet.local();

  const confirmOptions: ConfirmOptions = {
    commitment: "confirmed",
    preflightCommitment: "confirmed"
  };
  
  anchor.setProvider(
    new anchor.AnchorProvider(
      connection,
      wallet,
      confirmOptions
    )
  );

  const program = anchor.workspace.NftAuction as Program<NftAuction>;
});

