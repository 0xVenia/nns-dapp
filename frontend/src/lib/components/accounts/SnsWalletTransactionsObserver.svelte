<script lang="ts">
  import type { TransactionsObserverData } from "$lib/types/icrc.observer";
  import { isNullish, nonNullish } from "@dfinity/utils";
  import {
    snsOnlyProjectStore,
    snsProjectSelectedStore,
  } from "$lib/derived/sns/sns-selected-project.derived";
  import TransactionsObserver from "$lib/components/accounts/TransactionsObserver.svelte";
  import type { UniverseCanisterId } from "$lib/types/universe";
  import type { TransactionsCallback } from "$lib/services/worker-transactions.services";
  import type { Account } from "$lib/types/account";
  import { addObservedIcrcTransactionsToStore } from "$lib/services/observer.services";

  export let account: Account;
  export let completed: boolean;

  let data: TransactionsObserverData | undefined;
  $: data = nonNullish($snsProjectSelectedStore)
    ? {
        account,
        indexCanisterId:
          $snsProjectSelectedStore.summary.indexCanisterId.toText(),
      }
    : undefined;

  const callback: TransactionsCallback = ({ transactions }) => {
    if (isNullish(universeId)) {
      // With current usage, can unlikely be undefined here
      return;
    }

    addObservedIcrcTransactionsToStore({
      universeId,
      completed,
      transactions,
    });
  };

  let universeId: UniverseCanisterId | undefined;
  $: universeId = $snsOnlyProjectStore;
</script>

{#if nonNullish(data) && nonNullish(universeId)}
  <TransactionsObserver {data} {callback}>
    <slot />
  </TransactionsObserver>
{/if}
