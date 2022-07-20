/**
 * @jest-environment jsdom
 */

import ProjectSwapDetails from "../../../../lib/components/project-detail/ProjectSwapDetails.svelte";
import type { SnsSwapCommitment } from "../../../../lib/types/sns";
import {
  mockSnsFullProject,
  mockSummary,
  mockSwapInit,
} from "../../../mocks/sns-projects.mock";
import { renderContextCmp } from "../../../mocks/sns.mock";

describe("ProjectSwapDetails", () => {
  it("should render min commitment", () => {
    const { container } = renderContextCmp({
      summary: mockSummary,
      swapCommitment: mockSnsFullProject.swapCommitment as SnsSwapCommitment,
      Component: ProjectSwapDetails,
    });

    const element = Array.from(
      container.querySelectorAll('[data-tid="icp-value"]')
    )[0];

    expect(element?.innerHTML).toEqual(
      `${(Number(mockSwapInit.min_participant_icp_e8s) / 100000000).toFixed(2)}`
    );
  });

  it("should render max commitment", () => {
    const { container } = renderContextCmp({
      summary: mockSummary,
      swapCommitment: mockSnsFullProject.swapCommitment as SnsSwapCommitment,
      Component: ProjectSwapDetails,
    });

    const element = Array.from(
      container.querySelectorAll('[data-tid="icp-value"]')
    )[1];

    expect(element?.innerHTML).toEqual(
      `${(Number(mockSwapInit.max_participant_icp_e8s) / 100000000).toFixed(2)}`
    );
  });
});
