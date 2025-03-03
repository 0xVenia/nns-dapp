<script lang="ts">
  import {
    IconExplore,
    IconUsers,
    IconPassword,
    IconRocketLaunch,
    IconWallet,
    MenuItem,
  } from "@dfinity/gix-components";
  import type { ComponentType } from "svelte";
  import { i18n } from "$lib/stores/i18n";
  import { AppPath } from "$lib/constants/routes.constants";
  import { IS_TESTNET } from "$lib/constants/environment.constants";
  import TestIdWrapper from "$lib/components/common/TestIdWrapper.svelte";
  import GetTokens from "$lib/components/ic/GetTokens.svelte";
  import {
    canistersPathStore,
    neuronsPathStore,
    tokensPathStore,
    proposalsPathStore,
  } from "$lib/derived/paths.derived";
  import { keyOf } from "$lib/utils/utils";
  import { pageStore } from "$lib/derived/page.derived";
  import { isSelectedPath } from "$lib/utils/navigation.utils";
  import MenuMetrics from "$lib/components/common/MenuMetrics.svelte";

  let routes: {
    context: string;
    href: string;
    selected: boolean;
    label: string;
    icon:
      | typeof IconWallet
      | typeof IconPassword
      | typeof IconUsers
      | typeof IconRocketLaunch
      | typeof IconExplore;
    statusIcon?: ComponentType;
  }[];
  $: routes = [
    {
      context: "accounts",
      href: $tokensPathStore,
      selected: isSelectedPath({
        currentPath: $pageStore.path,
        paths: [AppPath.Accounts, AppPath.Wallet, AppPath.Tokens],
      }),
      label: "tokens",
      icon: IconWallet,
    },
    {
      context: "neurons",
      href: $neuronsPathStore,
      selected: isSelectedPath({
        currentPath: $pageStore.path,
        paths: [AppPath.Neurons, AppPath.Neuron],
      }),
      label: "neurons",
      icon: IconPassword,
    },
    {
      context: "proposals",
      href: $proposalsPathStore,
      selected: isSelectedPath({
        currentPath: $pageStore.path,
        paths: [AppPath.Proposals, AppPath.Proposal],
      }),
      label: "voting",
      icon: IconUsers,
    },
    {
      context: "launchpad",
      href: `${AppPath.Launchpad}`,
      selected: isSelectedPath({
        currentPath: $pageStore.path,
        paths: [AppPath.Launchpad, AppPath.Project],
      }),
      label: "launchpad",
      icon: IconRocketLaunch,
    },
    {
      context: "canisters",
      href: $canistersPathStore,
      selected: isSelectedPath({
        currentPath: $pageStore.path,
        paths: [AppPath.Canisters, AppPath.Canister],
      }),
      label: "canisters",
      icon: IconExplore,
    },
  ];
</script>

<TestIdWrapper testId="menu-items-component">
  {#each routes as { context, label, href, icon, statusIcon, selected } (context)}
    {@const title = keyOf({ obj: $i18n.navigation, key: label })}

    <MenuItem {href} testId={`menuitem-${context}`} {selected} {title}>
      <svelte:component this={icon} slot="icon" />
      <svelte:fragment>{title}</svelte:fragment>
      <svelte:component this={statusIcon} slot="statusIcon" />
    </MenuItem>
  {/each}

  {#if IS_TESTNET}
    <GetTokens />
  {/if}

  <MenuMetrics />
</TestIdWrapper>
