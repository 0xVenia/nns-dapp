<script lang="ts">
  import { TokenAmountV2 } from "@dfinity/utils";
  import IcrcTokenTransactionModal from "./IcrcTokenTransactionModal.svelte";
  import type { IcrcTokenModalProps } from "$lib/types/icrc-accounts.modal";

  let modal: IcrcTokenModalProps | undefined = undefined;

  const closeModal = () => {
    modal = undefined;
  };

  const onIcrcTokenAccountsModal = ({
    detail,
  }: CustomEvent<IcrcTokenModalProps>) => {
    modal = detail;
  };
</script>

<svelte:window on:nnsIcrcTokenModal={onIcrcTokenAccountsModal} />

{#if modal?.type === "icrc-send"}
  <IcrcTokenTransactionModal
    on:nnsClose={closeModal}
    ledgerCanisterId={modal.data.universeId}
    token={modal.data.token}
    transactionFee={TokenAmountV2.fromUlps({
      amount: modal.data.token.fee,
      token: modal.data.token,
    })}
    selectedAccount={modal.data.sourceAccount}
    reloadSourceAccount={modal.data.reloadSourceAccount}
  />
{/if}
