import { snsAggregatorStore } from "$lib/stores/sns-aggregator.store";
import { snsDerivedStateStore } from "$lib/stores/sns-derived-state.store";
import { snsLifecycleStore } from "$lib/stores/sns-lifecycle.store";
import type { IcrcTokenMetadata } from "$lib/types/icrc";
import { aggregatorSnsMockWith } from "$tests/mocks/sns-aggregator.mock";
import { principal } from "$tests/mocks/sns-projects.mock";
import type { Principal } from "@dfinity/principal";
import type { SnsNervousSystemFunction, SnsSwapLifecycle } from "@dfinity/sns";

export const setSnsProjects = (
  params: {
    rootCanisterId?: Principal;
    lifecycle: SnsSwapLifecycle;
    certified?: boolean;
    restrictedCountries?: string[];
    directParticipantCount?: [] | [bigint];
    projectName?: string;
    tokenMetadata?: Partial<IcrcTokenMetadata>;
    nervousFunctions?: SnsNervousSystemFunction[];
    swapDueTimestampSeconds?: number;
    nnsProposalId?: number;
  }[]
) => {
  const aggregatorProjects = params.map((params, index) => {
    return aggregatorSnsMockWith({
      rootCanisterId:
        params.rootCanisterId?.toText() ?? principal(index).toText(),
      lifecycle: params.lifecycle,
      restrictedCountries: params.restrictedCountries,
      directParticipantCount: params.directParticipantCount,
      projectName: params.projectName,
      tokenMetadata: params.tokenMetadata,
      nervousFunctions: params.nervousFunctions,
      swapDueTimestampSeconds: params.swapDueTimestampSeconds,
      nnsProposalId: params.nnsProposalId,
    });
  });
  snsLifecycleStore.reset();
  snsDerivedStateStore.reset();
  snsAggregatorStore.setData(aggregatorProjects);
};

export const resetSnsProjects = () => {
  snsLifecycleStore.reset();
  snsDerivedStateStore.reset();
  snsAggregatorStore.reset();
};
