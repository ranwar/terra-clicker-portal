// module.exports = ({ wallets, refs, config, client }) => ({
//   getCount: () => client.query("counter", { get_count: {} }),
//   increment: (signer = wallets.validator) =>
//     client.execute(signer, "counter", { increment: {} }),

//     // This is what we're adding. "clicker" is the contract we want to interact with, "getSpeed" is the function.
//     getSpeed: () => client.query("clicker", { get_speed: {} }),
// });

module.exports = ({ wallets, refs, config, client }) => ({
  getSpeed: () => client.query("clicker", { get_speed: {} }),
  getScores: () => client.query("clicker", { get_scores: {} }),

  upsertScore: (score, signer = wallets.validator) =>
    client.execute(signer, "clicker", { upsert_score: { score } }),
});