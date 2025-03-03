<script lang="ts" xmlns:svelte="http://www.w3.org/1999/html">
  import CurrentBalance from "$lib/components/accounts/CurrentBalance.svelte";
  import { Modal, Value, busy } from "@dfinity/gix-components";
  import type { NeuronInfo } from "@dfinity/nns";
  import { ICPToken, TokenAmountV2 } from "@dfinity/utils";
  import { isValidInputAmount, neuronStake } from "$lib/utils/neuron.utils";
  import AmountInput from "$lib/components/ui/AmountInput.svelte";
  import { i18n } from "$lib/stores/i18n";
  import {
    formattedTransactionFeeICP,
    ulpsToNumber,
  } from "$lib/utils/token.utils";
  import { startBusy, stopBusy } from "$lib/stores/busy.store";
  import { createEventDispatcher } from "svelte";
  import { splitNeuron } from "$lib/services/neurons.services";
  import { toastsError, toastsSuccess } from "$lib/stores/toasts.store";
  import { mainTransactionFeeStore } from "$lib/stores/transaction-fees.store";

  export let neuron: NeuronInfo;

  let amount: number | undefined;

  let stakeE8s: bigint;
  $: stakeE8s = neuronStake(neuron);

  let balance: TokenAmountV2;
  $: balance = TokenAmountV2.fromUlps({ amount: stakeE8s, token: ICPToken });

  let max = 0;
  $: max =
    stakeE8s === 0n
      ? 0
      : ulpsToNumber({
          ulps: stakeE8s - BigInt($mainTransactionFeeStore),
          token: ICPToken,
        });

  let validForm: boolean;
  $: validForm = isValidInputAmount({ amount, max });

  const onMax = () => (amount = max);

  const dispatcher = createEventDispatcher();
  const close = () => dispatcher("nnsClose");
  const split = async () => {
    // TS is not smart enought to understand that `validForm` also covers `amount === undefined`
    if (!validForm || amount === undefined) {
      toastsError({
        labelKey: "error.amount_not_valid",
      });
      return;
    }
    startBusy({ initiator: "split-neuron" });

    const id = await splitNeuron({
      neuron,
      amount,
    });
    if (id !== undefined) {
      toastsSuccess({
        labelKey: "neuron_detail.split_neuron_success",
      });
    }
    close();
    stopBusy("split-neuron");
  };
</script>

<Modal on:nnsClose>
  <svelte:fragment slot="title"
    >{$i18n.neuron_detail.split_neuron}</svelte:fragment
  >
  <div class="wrapper" data-tid="split-neuron-modal">
    <CurrentBalance {balance} />

    <AmountInput bind:amount on:nnsMax={onMax} {max} />

    <div>
      <p class="label">{$i18n.neurons.transaction_fee}</p>
      <p>
        <Value>{formattedTransactionFeeICP($mainTransactionFeeStore)}</Value> ICP
      </p>
    </div>

    <div class="toolbar">
      <button class="secondary" on:click={close}>
        {$i18n.core.cancel}
      </button>
      <button
        data-tid="split-neuron-button"
        class="primary"
        on:click={split}
        disabled={!validForm || $busy}
      >
        {$i18n.neuron_detail.split_neuron_confirm}
      </button>
    </div>
  </div>
</Modal>

<style lang="scss">
  .wrapper {
    display: flex;
    flex-direction: column;
    gap: var(--padding);
  }
</style>
