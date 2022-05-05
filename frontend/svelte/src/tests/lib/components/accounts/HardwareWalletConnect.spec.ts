/**
 * @jest-environment jsdom
 */

import { fireEvent } from "@testing-library/dom";
import { render, waitFor } from "@testing-library/svelte";
import HardwareWalletConnect from "../../../../lib/components/accounts/HardwareWalletConnect.svelte";
import { LedgerConnectionState } from "../../../../lib/constants/ledger.constants";
import { connectToHardwareWalletProxy } from "../../../../lib/proxy/ledger.services.proxy";
import { addAccountStore } from "../../../../lib/stores/add-account.store";
import { mockIdentity } from "../../../mocks/auth.store.mock";
import AddAccountTest from "./AddAccountTest.svelte";

jest.mock("../../../../lib/proxy/ledger.services.proxy");

describe("HardwareWalletConnect", () => {
  const props = { testComponent: HardwareWalletConnect };

  beforeAll(() => {
    addAccountStore.set({
      type: "hardwareWallet",
      hardwareWalletName: undefined,
    });

    (connectToHardwareWalletProxy as jest.Mock).mockImplementation(
      async (callback) =>
        callback({
          connectionState: LedgerConnectionState.CONNECTED,
          ledgerIdentity: mockIdentity,
        })
    );
  });

  afterAll(() =>
    addAccountStore.set({
      type: undefined,
      hardwareWalletName: undefined,
    })
  );

  it("should render a connect action", () => {
    const { getByTestId } = render(AddAccountTest, {
      props,
    });

    expect(getByTestId("ledger-connect-button")).not.toBeNull();
  });

  it("should not enable attach action if not connected", () => {
    const { getByTestId } = render(AddAccountTest, {
      props,
    });

    const button = getByTestId("ledger-attach-button") as HTMLButtonElement;

    expect(button).not.toBeNull();
    expect(button.getAttribute("disabled")).not.toBeNull();
  });

  it("should enable attach action if connected", async () => {
    const { getByTestId } = render(AddAccountTest, {
      props,
    });

    const connect = getByTestId("ledger-connect-button") as HTMLButtonElement;

    fireEvent.click(connect);

    await waitFor(() => {
      const button = getByTestId("ledger-attach-button") as HTMLButtonElement;

      expect(button.getAttribute("disabled")).toBeNull();
    });
  });
});
