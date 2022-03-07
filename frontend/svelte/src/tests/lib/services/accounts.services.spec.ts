import { ICP, LedgerCanister } from "@dfinity/nns";
import { mock } from "jest-mock-extended";
import { readable } from "svelte/store";
import { NNSDappCanister } from "../../../lib/canisters/nns-dapp/nns-dapp.canister";
import type { AccountDetails } from "../../../lib/canisters/nns-dapp/nns-dapp.types";
import * as services from "../../../lib/services/accounts.services";
import {
  mockAccountDetails,
  mockSubAccountDetails,
} from "../../mocks/accounts.store.mock";
import { mockIdentity } from "../../mocks/auth.store.mock";

jest.mock("../../../lib/stores/auth.store", () => {
  return {
    authStore: readable({
      identity: {
        getPrincipal: jest.fn(),
      },
    }),
  };
});

describe("accounts-services", () => {
  it("should call ledger and nnsdapp to get account and balance", async () => {
    // Ledger mock
    const ledgerMock = mock<LedgerCanister>();
    ledgerMock.accountBalance.mockResolvedValue(ICP.fromString("1") as ICP);
    jest
      .spyOn(LedgerCanister, "create")
      .mockImplementation((): LedgerCanister => ledgerMock);

    // NNSDapp mock
    const nnsDappMock = mock<NNSDappCanister>();
    nnsDappMock.getAccount.mockResolvedValue(mockAccountDetails);
    jest.spyOn(NNSDappCanister, "create").mockImplementation(() => nnsDappMock);

    await services.syncAccounts({ identity: mockIdentity });

    expect(ledgerMock.accountBalance).toBeCalled();
    expect(nnsDappMock.getAccount).toBeCalled();
    expect(nnsDappMock.addAccount).toBeCalledTimes(1);
  });

  it("should get balances of subaccounts", async () => {
    // Ledger mock
    const ledgerMock = mock<LedgerCanister>();
    ledgerMock.accountBalance.mockResolvedValue(ICP.fromString("1") as ICP);
    jest
      .spyOn(LedgerCanister, "create")
      .mockImplementation((): LedgerCanister => ledgerMock);

    // NNSDapp mock
    const nnsDappMock = mock<NNSDappCanister>();
    const accountDetails: AccountDetails = {
      ...mockAccountDetails,
      sub_accounts: [mockSubAccountDetails],
    };
    nnsDappMock.getAccount.mockResolvedValue(accountDetails);
    jest.spyOn(NNSDappCanister, "create").mockImplementation(() => nnsDappMock);

    await services.syncAccounts({ identity: mockIdentity });

    // Called once for main, another for the subaccount = 2
    expect(ledgerMock.accountBalance).toBeCalledTimes(2);
  });

  it("should call nnsDappCanister to create subaccount", async () => {
    const nnsDappMock = mock<NNSDappCanister>();
    jest.spyOn(NNSDappCanister, "create").mockImplementation(() => nnsDappMock);
    const mockSyncAccounts = jest.fn();
    jest.spyOn(services, "syncAccounts").mockImplementation(mockSyncAccounts);

    await services.createSubAccount("test subaccount");

    expect(nnsDappMock.createSubAccount).toBeCalled();
  });
});
