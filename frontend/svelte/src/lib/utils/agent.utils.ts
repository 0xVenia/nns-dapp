import { HttpAgent, Identity } from "@dfinity/agent";

export const createAgent = async ({
  identity,
  host,
}: {
  identity: Identity;
  host?: string;
}): Promise<HttpAgent> => {
  const agent: HttpAgent = new HttpAgent({
    identity,
    ...(host !== undefined && { host }),
  });

  // process.env.FETCH_ROOT_KEY is changed to `true`, but we hande nullish/empty cases explicitly
  // @ts-ignore
  if (process.env.FETCH_ROOT_KEY === true) {
    // Fetch root key for certificate validation during development or on testnet
    await agent.fetchRootKey();
  }

  return agent;
};
