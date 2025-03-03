<script lang="ts">
  import { i18n } from "$lib/stores/i18n";
  import { openSnsNeuronModal } from "$lib/utils/modals.utils";
  import type { SnsNeuron } from "@dfinity/sns";
  import {
    isVesting,
    hasEnoughStakeToSplit,
  } from "$lib/utils/sns-neuron.utils";
  import type { SnsNervousSystemParameters } from "@dfinity/sns";
  import { fromDefinedNullable } from "@dfinity/utils";
  import { minNeuronSplittable } from "$lib/utils/sns-neuron.utils";
  import { replacePlaceholders } from "$lib/utils/i18n.utils";
  import type { E8s } from "@dfinity/nns";
  import { formatTokenE8s } from "$lib/utils/token.utils";
  import Tooltip from "$lib/components/ui/Tooltip.svelte";
  import type { Token } from "@dfinity/utils";
  import VestingTooltipWrapper from "../VestingTooltipWrapper.svelte";

  export let neuron: SnsNeuron;
  export let parameters: SnsNervousSystemParameters;
  export let transactionFee: E8s;
  export let token: Token;

  let neuronMinimumStake: bigint;
  $: neuronMinimumStake = fromDefinedNullable(
    parameters.neuron_minimum_stake_e8s
  );

  let enoughStakeToSplit: boolean;
  $: enoughStakeToSplit = hasEnoughStakeToSplit({
    neuron,
    fee: transactionFee,
    neuronMinimumStake,
  });
</script>

{#if enoughStakeToSplit}
  <VestingTooltipWrapper {neuron}>
    <button
      class="secondary"
      disabled={isVesting(neuron)}
      on:click={() => openSnsNeuronModal({ type: "split-neuron" })}
      data-tid="split-neuron-button">{$i18n.neuron_detail.split_neuron}</button
    >
  </VestingTooltipWrapper>
{:else}
  <Tooltip
    id="split-neuron-button"
    text={replacePlaceholders(
      $i18n.neuron_detail.split_neuron_disabled_tooltip,
      {
        $amount: formatTokenE8s({
          value: minNeuronSplittable({
            fee: transactionFee,
            neuronMinimumStake,
          }),
          detailed: true,
        }),
        $token: token.symbol,
      }
    )}
  >
    <button class="secondary" data-tid="split-neuron-button" disabled
      >{$i18n.neuron_detail.split_neuron}</button
    >
  </Tooltip>
{/if}
