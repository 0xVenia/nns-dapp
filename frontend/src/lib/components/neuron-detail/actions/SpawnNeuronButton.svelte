<script lang="ts">
  import type { NeuronInfo } from "@dfinity/nns";
  import { ULPS_PER_MATURITY } from "$lib/constants/neurons.constants";
  import {
    MIN_NEURON_STAKE,
    MATURITY_MODULATION_VARIANCE_PERCENTAGE,
  } from "$lib/constants/neurons.constants";
  import { i18n } from "$lib/stores/i18n";
  import { formatNumber, formatPercentage } from "$lib/utils/format.utils";
  import { replacePlaceholders } from "$lib/utils/i18n.utils";
  import { isEnoughMaturityToSpawn } from "$lib/utils/neuron.utils";
  import Tooltip from "$lib/components/ui/Tooltip.svelte";
  import {
    NNS_NEURON_CONTEXT_KEY,
    type NnsNeuronContext,
  } from "$lib/types/nns-neuron-detail.context";
  import { getContext } from "svelte";
  import { openNnsNeuronModal } from "$lib/utils/modals.utils";
  import TestIdWrapper from "$lib/components/common/TestIdWrapper.svelte";

  export let neuron: NeuronInfo;

  let enoughMaturity: boolean;
  $: enoughMaturity =
    neuron.fullNeuron === undefined
      ? false
      : isEnoughMaturityToSpawn({
          neuron,
          percentage: 100,
        });

  const { store }: NnsNeuronContext = getContext<NnsNeuronContext>(
    NNS_NEURON_CONTEXT_KEY
  );

  const showModal = () =>
    openNnsNeuronModal({ type: "spawn", data: { neuron: $store.neuron } });
</script>

<TestIdWrapper testId="spawn-neuron-button-component">
  {#if enoughMaturity}
    <button class="secondary" on:click={showModal}>
      {$i18n.neuron_detail.spawn_neuron}
    </button>
  {:else}
    <Tooltip
      id="spawn-maturity-button"
      text={replacePlaceholders(
        $i18n.neuron_detail.spawn_neuron_disabled_tooltip,
        {
          $amount: formatNumber(
            MIN_NEURON_STAKE /
              ULPS_PER_MATURITY /
              MATURITY_MODULATION_VARIANCE_PERCENTAGE,
            { minFraction: 4, maxFraction: 4 }
          ),
          $min: formatNumber(MIN_NEURON_STAKE / ULPS_PER_MATURITY, {
            minFraction: 0,
            maxFraction: 0,
          }),
          $variability: formatPercentage(
            MATURITY_MODULATION_VARIANCE_PERCENTAGE,
            {
              minFraction: 0,
              maxFraction: 0,
            }
          ),
        }
      )}
    >
      <button disabled class="secondary">
        {$i18n.neuron_detail.spawn_neuron}
      </button>
    </Tooltip>
  {/if}
</TestIdWrapper>
