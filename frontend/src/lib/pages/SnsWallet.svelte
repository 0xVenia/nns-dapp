<script lang="ts">
  import { buildAccountsUrl } from "$lib/utils/navigation.utils";
  import { goto } from "$app/navigation";
  import { hasAccounts } from "$lib/utils/accounts.utils";
  import type { Principal } from "@dfinity/principal";
  import { Spinner, busy } from "@dfinity/gix-components";
  import { setContext } from "svelte";
  import { writable } from "svelte/store";
  import { snsProjectAccountsStore } from "$lib/derived/sns/sns-project-accounts.derived";
  import { syncSnsAccounts } from "$lib/services/sns-accounts.services";
  import { debugSelectedAccountStore } from "$lib/derived/debug.derived";
  import {
    WALLET_CONTEXT_KEY,
    type WalletContext,
    type WalletStore,
  } from "$lib/types/wallet.context";
  import TestIdWrapper from "$lib/components/common/TestIdWrapper.svelte";
  import Footer from "$lib/components/layout/Footer.svelte";
  import { i18n } from "$lib/stores/i18n";
  import SnsTransactionModal from "$lib/modals/accounts/SnsTransactionModal.svelte";
  import SnsTransactionsList from "$lib/components/accounts/SnsTransactionsList.svelte";
  import Separator from "$lib/components/ui/Separator.svelte";
  import { Island } from "@dfinity/gix-components";
  import {
    snsOnlyProjectStore,
    snsProjectSelectedStore,
  } from "$lib/derived/sns/sns-selected-project.derived";
  import { TokenAmount, isNullish, nonNullish } from "@dfinity/utils";
  import { selectedUniverseStore } from "$lib/derived/selected-universe.derived";
  import { loadSnsAccountTransactions } from "$lib/services/sns-transactions.services";
  import { replacePlaceholders } from "$lib/utils/i18n.utils";
  import { toastsError } from "$lib/stores/toasts.store";
  import ReceiveButton from "$lib/components/accounts/ReceiveButton.svelte";
  import { tokensStore } from "$lib/stores/tokens.store";
  import type { IcrcTokenMetadata } from "$lib/types/icrc";
  import SnsBalancesObserver from "$lib/components/accounts/SnsBalancesObserver.svelte";
  import WalletPageHeader from "$lib/components/accounts/WalletPageHeader.svelte";
  import WalletPageHeading from "$lib/components/accounts/WalletPageHeading.svelte";
  import { snsSelectedTransactionFeeStore } from "$lib/derived/sns/sns-selected-transaction-fee.store";
  import IC_LOGO from "$lib/assets/icp.svg";
  import { toTokenAmountV2 } from "$lib/utils/token.utils";

  let showModal: "send" | undefined = undefined;

  const onSnsProjectChanged = async (
    selectedProjectCanisterId: Principal | undefined
  ) => {
    if (selectedProjectCanisterId !== undefined) {
      // Reload accounts always.
      // Do not set to loading because we might use the account in the store.
      await syncSnsAccounts({ rootCanisterId: selectedProjectCanisterId });
    }
  };

  $: onSnsProjectChanged($snsOnlyProjectStore);

  const selectedAccountStore = writable<WalletStore>({
    account: undefined,
    neurons: [],
  });

  debugSelectedAccountStore(selectedAccountStore);

  setContext<WalletContext>(WALLET_CONTEXT_KEY, {
    store: selectedAccountStore,
  });

  const goBack = (): Promise<void> =>
    goto(
      buildAccountsUrl({
        universe: $selectedUniverseStore.canisterId,
      })
    );

  export let accountIdentifier: string | undefined | null = undefined;

  const load = () => {
    if (nonNullish(accountIdentifier)) {
      const selectedAccount = $snsProjectAccountsStore?.find(
        ({ identifier }) => identifier === accountIdentifier
      );

      selectedAccountStore.set({
        account: selectedAccount,
        neurons: [],
      });
    }
    // Accounts are loaded in store but no account identifier is matching
    if (
      hasAccounts($snsProjectAccountsStore ?? []) &&
      isNullish($selectedAccountStore.account)
    ) {
      toastsError({
        labelKey: replacePlaceholders($i18n.error.account_not_found, {
          $account_identifier: accountIdentifier ?? "",
        }),
      });

      goBack();
    }
  };

  const reloadTransactions = async () => {
    if (
      isNullish($selectedAccountStore.account) ||
      isNullish($snsOnlyProjectStore)
    ) {
      return;
    }

    await loadSnsAccountTransactions({
      account: $selectedAccountStore.account,
      canisterId: $snsOnlyProjectStore,
    });
  };

  $: accountIdentifier, $snsProjectAccountsStore, load();

  let disabled = false;
  $: disabled = isNullish($selectedAccountStore.account) || $busy;

  const reloadAccount = async () => {
    try {
      await Promise.all([
        nonNullish($snsOnlyProjectStore)
          ? syncSnsAccounts({ rootCanisterId: $snsOnlyProjectStore })
          : Promise.resolve(),
        reloadTransactions(),
      ]);

      // Apply reloaded values - balance - to UI
      load();
    } catch (err: unknown) {
      toastsError({
        labelKey: replacePlaceholders($i18n.error.account_not_reload, {
          $account_identifier: accountIdentifier ?? "",
        }),
        err,
      });
    }
  };

  let token: IcrcTokenMetadata | undefined;
  $: token = nonNullish($snsOnlyProjectStore)
    ? $tokensStore[$snsOnlyProjectStore.toText()]?.token
    : undefined;
</script>

<TestIdWrapper testId="sns-wallet-component">
  <Island>
    <main class="legacy" data-tid="sns-wallet">
      <section>
        {#if nonNullish($selectedAccountStore.account) && nonNullish($snsOnlyProjectStore) && nonNullish($snsProjectSelectedStore) && nonNullish(token)}
          <SnsBalancesObserver
            rootCanisterId={$snsOnlyProjectStore}
            accounts={[$selectedAccountStore.account]}
            ledgerCanisterId={$snsProjectSelectedStore.summary.ledgerCanisterId}
          >
            <WalletPageHeader
              universe={$selectedUniverseStore}
              walletAddress={$selectedAccountStore.account.identifier}
            />
            <WalletPageHeading
              balance={TokenAmount.fromE8s({
                amount: $selectedAccountStore.account.balanceUlps,
                token,
              })}
              accountName={$selectedAccountStore.account.name ??
                $i18n.accounts.main}
            />

            <Separator spacing="none" />

            <SnsTransactionsList
              rootCanisterId={$snsOnlyProjectStore}
              account={$selectedAccountStore.account}
              {token}
            />
          </SnsBalancesObserver>
        {:else}
          <Spinner />
        {/if}
      </section>
    </main>

    <Footer>
      <button
        class="primary"
        on:click={() => (showModal = "send")}
        {disabled}
        data-tid="open-new-sns-transaction">{$i18n.accounts.send}</button
      >

      <ReceiveButton
        type="icrc-receive"
        account={$selectedAccountStore.account}
        reload={reloadAccount}
        testId="receive-sns"
        universeId={$snsOnlyProjectStore}
        logo={$selectedUniverseStore?.summary?.metadata.logo ?? IC_LOGO}
        tokenSymbol={$selectedUniverseStore?.summary?.token.symbol}
      />
    </Footer>
  </Island>

  {#if showModal && nonNullish($snsOnlyProjectStore)}
    <SnsTransactionModal
      on:nnsClose={() => (showModal = undefined)}
      selectedAccount={$selectedAccountStore.account}
      rootCanisterId={$snsOnlyProjectStore}
      loadTransactions
      {token}
      transactionFee={toTokenAmountV2($snsSelectedTransactionFeeStore)}
    />
  {/if}
</TestIdWrapper>

<style lang="scss">
  section {
    display: flex;
    flex-direction: column;
    gap: var(--padding-4x);
  }
</style>
